/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_mut
)]

use crate::DisplayExt;
use std::ffi::CStr;
use crate::dpx_pdfobj::PdfObjRef;

use crate::info;

use super::dpx_agl::agl_get_unicodes;
use super::dpx_dpxutil::parse_c_ident;
use super::dpx_mem::new;
use super::dpx_pdfparse::skip_white;
use crate::dpx_pdfobj::{
    pdf_add_array, pdf_add_dict, pdf_array_length, pdf_copy_name, pdf_get_array, pdf_link_obj,
    pdf_lookup_dict, pdf_new_array, pdf_new_dict, pdf_new_null, pdf_new_number, pdf_new_string,
    pdf_obj, pdf_ref_obj, pdf_release_obj, pdf_string_value,
};
use crate::streq_ptr;
use crate::{ttstub_input_close, ttstub_input_get_size, ttstub_input_open, ttstub_input_read};
use libc::{free, memset, strcat, strcmp, strcpy, strlen};

pub type __ssize_t = i64;
pub type size_t = u64;
pub type ssize_t = __ssize_t;

use crate::TTInputFormat;

/* quasi-hack to get the primary input */
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
static mut verbose: i32 = 0i32;
#[no_mangle]
pub unsafe extern "C" fn otl_conf_set_verbose(mut level: i32) {
    verbose = level;
}
unsafe fn parse_uc_coverage(
    mut gclass: PdfObjRef,
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> PdfObjRef {
    let mut ucv: i32 = 0i32;
    if (*pp).offset(1) >= endptr {
        return 0 as PdfObjRef;
    }
    if **pp as i32 == '[' as i32 {
        *pp = (*pp).offset(1)
    }
    let coverage = pdf_new_array();
    while *pp < endptr {
        skip_white(pp, endptr);
        match **pp as i32 {
            93 | 59 => {
                *pp = (*pp).offset(1);
                return coverage;
            }
            44 => *pp = (*pp).offset(1),
            64 => {
                *pp = (*pp).offset(1);
                let glyphclass = CStr::from_ptr(parse_c_ident(pp, endptr));
                let cvalues = pdf_lookup_dict(&mut *gclass, glyphclass.to_bytes())
                    .expect(&format!("{} not defined...", glyphclass.display()));
                let size = pdf_array_length(&*cvalues) as i32;
                for i in 0..size {
                    pdf_add_array(&mut *coverage, pdf_link_obj(pdf_get_array(&mut *cvalues, i)));
                }
            }
            _ => {
                let mut glyphname = parse_c_ident(pp, endptr);
                if glyphname.is_null() {
                    panic!("Invalid Unicode character specified.");
                }
                skip_white(pp, endptr);
                let value;
                if (*pp).offset(1) < endptr && **pp as i32 == '-' as i32 {
                    value = pdf_new_array();
                    if agl_get_unicodes(glyphname, &mut ucv, 1i32) != 1i32 {
                        panic!(
                            "Invalid Unicode char: {}",
                            CStr::from_ptr(glyphname).display(),
                        );
                    }
                    pdf_add_array(&mut *value, pdf_new_number(ucv as f64));
                    free(glyphname as *mut libc::c_void);
                    *pp = (*pp).offset(1);
                    skip_white(pp, endptr);
                    glyphname = parse_c_ident(pp, endptr);
                    if glyphname.is_null() {
                        panic!(
                            "Invalid Unicode char: {}",
                            CStr::from_ptr(glyphname).display(),
                        );
                    }
                    if agl_get_unicodes(glyphname, &mut ucv, 1i32) != 1i32 {
                        panic!(
                            "Invalid Unicode char: {}",
                            CStr::from_ptr(glyphname).display(),
                        );
                    }
                    pdf_add_array(&mut *value, pdf_new_number(ucv as f64));
                    free(glyphname as *mut libc::c_void);
                } else {
                    if agl_get_unicodes(glyphname, &mut ucv, 1i32) != 1i32 {
                        panic!(
                            "Invalid Unicode char: {}",
                            CStr::from_ptr(glyphname).display(),
                        );
                    }
                    value = pdf_new_number(ucv as f64);
                    free(glyphname as *mut libc::c_void);
                }
                pdf_add_array(&mut *coverage, value);
            }
        }
        skip_white(pp, endptr);
    }
    coverage
}
unsafe fn add_rule(
    mut rule: PdfObjRef,
    mut gclass: PdfObjRef,
    mut first: *mut i8,
    mut second: *mut i8,
    mut suffix: *mut i8,
) {
    let glyph1;
    let glyph2;
    let mut unicodes: [i32; 16] = [0; 16];
    if *first.offset(0) as i32 == '@' as i32 {
        let s = CStr::from_ptr(first.offset(1));
        let glyph1_opt = pdf_lookup_dict(&mut *gclass, s.to_bytes());
        if glyph1_opt.is_none() {
            warn!("No glyph class \"{}\" found.", s.display());
            return;
        }
        glyph1 = glyph1_opt.unwrap();
        pdf_link_obj(glyph1);
        if verbose > 0i32 {
            info!(
                "otl_conf>> Output glyph sequence: {}\n",
                CStr::from_ptr(first).display()
            );
        }
    } else {
        let n_unicodes = agl_get_unicodes(first, unicodes.as_mut_ptr(), 16i32);
        if n_unicodes < 1i32 {
            warn!(
                "Failed to convert glyph \"{}\" to Unicode sequence.",
                CStr::from_ptr(first).display()
            );
            return;
        }
        glyph1 = pdf_new_array();
        if verbose > 0i32 {
            info!(
                "otl_conf>> Output glyph sequence: {} ->",
                CStr::from_ptr(first).display()
            );
        }
        for i in 0..n_unicodes as usize {
            pdf_add_array(&mut *glyph1, pdf_new_number(unicodes[i] as f64));
            if verbose > 0i32 {
                if unicodes[i] < 0x10000i32 {
                    info!(" U+{:04X}", unicodes[i],);
                } else {
                    info!(" U+{:06X}", unicodes[i],);
                }
            }
        }
        if verbose > 0i32 {
            info!("\n");
        }
    }
    if *second.offset(0) as i32 == '@' as i32 {
        let s = CStr::from_ptr(second.offset(1));
        let glyph2_opt = pdf_lookup_dict(&mut *gclass, s.to_bytes());
        if glyph2_opt.is_none() {
            warn!("No glyph class \"{}\" found.", s.display(),);
            return;
        }
        glyph2 = glyph2_opt.unwrap();
        pdf_link_obj(glyph2);
        if verbose > 0i32 {
            info!(
                "otl_conf>> Input glyph sequence: {} ({})\n",
                CStr::from_ptr(second).display(),
                CStr::from_ptr(suffix).display(),
            );
        }
    } else {
        let n_unicodes = agl_get_unicodes(second, unicodes.as_mut_ptr(), 16i32);
        if n_unicodes < 1i32 {
            warn!(
                "Failed to convert glyph \"{}\" to Unicode sequence.",
                CStr::from_ptr(second).display()
            );
            return;
        }
        if verbose > 0i32 {
            if !suffix.is_null() {
                info!(
                    "otl_conf>> Input glyph sequence: {}.{} ->",
                    CStr::from_ptr(second).display(),
                    CStr::from_ptr(suffix).display(),
                );
            } else {
                info!(
                    "otl_conf>> Input glyph sequence: {} ->",
                    CStr::from_ptr(second).display()
                );
            }
        }
        glyph2 = pdf_new_array();
        for i in 0..n_unicodes as usize {
            pdf_add_array(&mut *glyph2, pdf_new_number(unicodes[i] as f64));
            if verbose > 0i32 {
                if unicodes[i] < 0x10000i32 {
                    info!(" U+{:04X}", unicodes[i],);
                } else {
                    info!(" U+{:06X}", unicodes[i],);
                }
            }
        }
        if verbose > 0i32 {
            info!(" ({})\n", CStr::from_ptr(suffix).display());
        }
    }
    /* OK */
    if !suffix.is_null() {
        pdf_add_array(
            &mut *rule,
            pdf_new_string(suffix as *const libc::c_void, strlen(suffix) as _),
        ); /* allows @ */
    } else {
        pdf_add_array(&mut *rule, pdf_new_null());
    }
    pdf_add_array(&mut *rule, glyph1);
    pdf_add_array(&mut *rule, glyph2);
}
unsafe fn parse_substrule(
    mut gclass: PdfObjRef,
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> PdfObjRef {
    skip_white(pp, endptr);
    if *pp < endptr && **pp as i32 == '{' as i32 {
        *pp = (*pp).offset(1)
    }
    skip_white(pp, endptr);
    if *pp >= endptr {
        return 0 as PdfObjRef;
    }
    let substrule = pdf_new_array();
    while *pp < endptr && **pp as i32 != '}' as i32 {
        skip_white(pp, endptr);
        if *pp >= endptr {
            break;
        }
        if **pp as i32 == '#' as i32 {
            while *pp < endptr {
                if **pp as i32 == '\r' as i32 || **pp as i32 == '\n' as i32 {
                    *pp = (*pp).offset(1);
                    break;
                } else {
                    *pp = (*pp).offset(1)
                }
            }
        } else if **pp as i32 == ';' as i32 {
            *pp = (*pp).offset(1)
        } else {
            skip_white(pp, endptr);
            let token = parse_c_ident(pp, endptr);
            if token.is_null() {
                break;
            }
            if streq_ptr(token, b"assign\x00" as *const u8 as *const i8) as i32 != 0
                || streq_ptr(token, b"substitute\x00" as *const u8 as *const i8) as i32 != 0
            {
                skip_white(pp, endptr);
                let first = parse_c_ident(pp, endptr);
                if first.is_null() {
                    panic!("Syntax error (1)");
                }
                skip_white(pp, endptr);
                let tmp = parse_c_ident(pp, endptr);
                if strcmp(tmp, b"by\x00" as *const u8 as *const i8) != 0
                    && strcmp(tmp, b"to\x00" as *const u8 as *const i8) != 0
                {
                    panic!("Syntax error (2): {}", CStr::from_ptr(*pp).display());
                }
                skip_white(pp, endptr);
                let second = parse_c_ident(pp, endptr);
                if second.is_null() {
                    panic!("Syntax error (3)");
                }
                /* (assign|substitute) tag dst src */
                pdf_add_array(&mut *substrule, pdf_copy_name(token)); /* = */
                let suffix = if (*pp).offset(1) < endptr && **pp as i32 == '.' as i32 {
                    *pp = (*pp).offset(1);
                    parse_c_ident(pp, endptr)
                } else {
                    0 as *mut i8
                };
                add_rule(substrule, gclass, first, second, suffix);
                free(first as *mut libc::c_void);
                free(tmp as *mut libc::c_void);
                free(second as *mut libc::c_void);
                free(suffix as *mut libc::c_void);
            } else {
                panic!("Unkown command {}.", CStr::from_ptr(token).display());
            }
            free(token as *mut libc::c_void);
            skip_white(pp, endptr);
        }
    }
    if *pp < endptr && **pp as i32 == '}' as i32 {
        *pp = (*pp).offset(1)
    }
    substrule
}
unsafe fn parse_block(
    mut gclass: PdfObjRef,
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> PdfObjRef {
    skip_white(pp, endptr);
    if *pp < endptr && **pp as i32 == '{' as i32 {
        *pp = (*pp).offset(1)
    }
    skip_white(pp, endptr);
    if *pp >= endptr {
        return 0 as PdfObjRef;
    }
    let rule = pdf_new_dict();
    while *pp < endptr && **pp as i32 != '}' as i32 {
        skip_white(pp, endptr);
        if *pp >= endptr {
            break;
        }
        if **pp as i32 == '#' as i32 {
            while *pp < endptr {
                if **pp as i32 == '\r' as i32 || **pp as i32 == '\n' as i32 {
                    *pp = (*pp).offset(1);
                    break;
                } else {
                    *pp = (*pp).offset(1)
                }
            }
        } else if **pp as i32 == ';' as i32 {
            *pp = (*pp).offset(1)
        } else {
            skip_white(pp, endptr);
            let token = parse_c_ident(pp, endptr);
            if token.is_null() {
                break;
            }
            let token_s = CStr::from_ptr(token).to_str().unwrap();
            if token_s == "script" || token_s == "language" {
                skip_white(pp, endptr);
                let mut len = 0;
                while (*pp).offset(len as isize) < endptr
                    && *(*pp).offset(len as isize) as i32 != ';' as i32
                {
                    len += 1
                }
                if len > 0i32 {
                    let tmp = new(((len + 1i32) as u32 as u64)
                        .wrapping_mul(::std::mem::size_of::<i8>() as u64)
                        as u32) as *mut i8;
                    memset(tmp as *mut libc::c_void, 0i32, (len + 1) as _);
                    for i in 0..len {
                        if libc::isspace(**pp as _) == 0 {
                            *tmp.offset(i as isize) = **pp
                        }
                        *pp = (*pp).offset(1);
                    }
                    pdf_add_dict(
                        &mut *rule,
                        token_s,
                        pdf_new_string(tmp as *const libc::c_void, strlen(tmp) as _),
                    );
                    if verbose > 0i32 {
                        info!(
                            "otl_conf>> Current {} set to \"{}\"\n",
                            token_s,
                            CStr::from_ptr(tmp).to_string_lossy(),
                        );
                    }
                    free(tmp as *mut libc::c_void);
                }
            } else if token_s == "option" {
                let opt_dict = pdf_lookup_dict(&mut *rule, "option").unwrap_or_else(|| {
                    let opt_dict = pdf_new_dict();
                    pdf_add_dict(&mut *rule, "option", opt_dict);
                    opt_dict
                });
                skip_white(pp, endptr);
                let tmp = parse_c_ident(pp, endptr);
                let tmp_s = CStr::from_ptr(tmp);
                if verbose > 0i32 {
                    info!("otl_conf>> Reading option \"{}\"\n", tmp_s.display(),);
                }
                skip_white(pp, endptr);
                let opt_rule = parse_block(gclass, pp, endptr);
                pdf_add_dict(&mut *opt_dict, tmp_s.to_bytes(), opt_rule);
                free(tmp as *mut libc::c_void);
            } else if token_s == "prefered" || token_s == "required" || token_s == "optional" {
                if verbose > 0i32 {
                    info!("otl_conf>> Reading block ({})\n", token_s,);
                }
                skip_white(pp, endptr);
                if *pp >= endptr || **pp as i32 != '{' as i32 {
                    panic!("Syntax error (1)");
                }
                let rule_block = parse_substrule(gclass, pp, endptr);
                let subst = pdf_lookup_dict(&mut *rule, "rule").unwrap_or_else(|| {
                    let subst = pdf_new_array();
                    pdf_add_dict(&mut *rule, "rule", subst);
                    subst
                });
                pdf_add_array(&mut *subst, pdf_new_number(*token.offset(0) as f64));
                pdf_add_array(&mut *subst, rule_block);
            } else if token_s.chars().nth(0) == Some('@') {
                skip_white(pp, endptr);
                *pp = (*pp).offset(1);
                skip_white(pp, endptr);
                if verbose > 0i32 {
                    info!("otl_conf>> Glyph class \"{}\"\n", token_s,);
                }
                let coverage = parse_uc_coverage(gclass, pp, endptr);
                if coverage.is_null() {
                    panic!("No valid Unicode characters...");
                }
                pdf_add_dict(&mut *gclass, &token_s[1..], coverage);
            }
            free(token as *mut libc::c_void);
            skip_white(pp, endptr);
        }
    }
    if *pp < endptr && **pp as i32 == '}' as i32 {
        *pp = (*pp).offset(1)
    }
    rule
}
unsafe fn otl_read_conf(mut conf_name: *const i8) -> PdfObjRef {
    let mut filename = new((strlen(conf_name)
        .wrapping_add(strlen(b".otl\x00" as *const u8 as *const i8))
        .wrapping_add(1))
    .wrapping_mul(::std::mem::size_of::<i8>()) as _) as *mut i8;
    strcpy(filename, conf_name);
    strcat(filename, b".otl\x00" as *const u8 as *const i8);
    let handle = ttstub_input_open(filename, TTInputFormat::CNF, 0i32);
    if handle.is_none() {
        free(filename as *mut libc::c_void);
        return 0 as PdfObjRef;
    }
    let mut handle = handle.unwrap();
    let mut size = ttstub_input_get_size(&mut handle) as i32;
    if verbose > 0i32 {
        info!("\n");
        info!(
            "otl_conf>> Layout config. \"{}\" found: file=\"{}\" ({} bytes)\n",
            CStr::from_ptr(conf_name).display(),
            CStr::from_ptr(filename).display(),
            size,
        );
    }
    free(filename as *mut libc::c_void);
    if size < 1i32 {
        return 0 as PdfObjRef;
    }
    let mut wbuf = new((size as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
        as *mut i8;
    let mut p = wbuf;
    let mut endptr = p.offset(size as isize);
    while size > 0i32 && p < endptr {
        let len = ttstub_input_read(handle.0.as_ptr(), p, size as size_t) as i32;
        if len < 0i32 {
            ttstub_input_close(handle);
            panic!(
                "error reading OTL configuration file \"{}\"",
                CStr::from_ptr(filename).display()
            );
        }
        p = p.offset(len as isize);
        size -= len
    }
    ttstub_input_close(handle);
    let mut pp = wbuf as *const i8;
    let gclass = pdf_new_dict();
    let rule = parse_block(gclass, &mut pp, endptr);
    pdf_release_obj(gclass);
    free(wbuf as *mut libc::c_void);
    rule
}
static mut otl_confs: PdfObjRef = 0 as *const pdf_obj as PdfObjRef;
#[no_mangle]
pub unsafe extern "C" fn otl_find_conf(mut _conf_name: *const i8) -> PdfObjRef {
    let mut _rule: PdfObjRef = 0 as PdfObjRef;
    let mut _script: PdfObjRef = 0 as PdfObjRef;
    let mut _language: PdfObjRef = 0 as PdfObjRef;
    let mut _options: PdfObjRef = 0 as PdfObjRef;
    0 as PdfObjRef
}
#[no_mangle]
pub unsafe extern "C" fn otl_conf_get_script(mut conf: PdfObjRef) -> *mut i8 {
    assert!(!conf.is_null());
    let script = pdf_lookup_dict(&mut *conf, "script").unwrap_or(0 as PdfObjRef);
    pdf_string_value(&*script) as *mut i8
}
#[no_mangle]
pub unsafe extern "C" fn otl_conf_get_language(mut conf: PdfObjRef) -> *mut i8 {
    assert!(!conf.is_null());
    let language = pdf_lookup_dict(&mut *conf, "language").unwrap_or(0 as PdfObjRef);
    pdf_string_value(&*language) as *mut i8
}
#[no_mangle]
pub unsafe extern "C" fn otl_conf_get_rule(mut conf: PdfObjRef) -> PdfObjRef {
    assert!(!conf.is_null());
    pdf_lookup_dict(&mut *conf, "rule").unwrap_or(0 as PdfObjRef)
}
#[no_mangle]
pub unsafe extern "C" fn otl_conf_find_opt(
    mut conf: PdfObjRef,
    mut opt_tag: *const i8,
) -> PdfObjRef {
    assert!(!conf.is_null());
    if let Some(options) = pdf_lookup_dict(&mut *conf, "option").filter(|_| !opt_tag.is_null()) {
        pdf_lookup_dict(&mut *options, CStr::from_ptr(opt_tag).to_bytes()).unwrap_or(0 as PdfObjRef)
    } else {
        0 as PdfObjRef
    }
}
#[no_mangle]
pub unsafe extern "C" fn otl_init_conf() {
    pdf_release_obj(otl_confs);
    otl_confs = pdf_new_dict();
    if verbose > 0i32 + 10i32 {
        pdf_release_obj(pdf_ref_obj(otl_confs));
    };
}
#[no_mangle]
pub unsafe extern "C" fn otl_close_conf() {
    pdf_release_obj(otl_confs);
    otl_confs = 0 as PdfObjRef;
}
