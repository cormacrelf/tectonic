#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

use crate::warn;

use crate::dpx_pdfobj::{
    pdf_add_array, pdf_add_dict, pdf_new_array, pdf_new_dict, pdf_new_name, pdf_new_number,
    pdf_new_string, pdf_obj,
};
use libc::free;
extern "C" {
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn rand() -> i32;
    #[no_mangle]
    fn srand(__seed: u32);
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn gmtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn MD5_init(ctx: *mut MD5_CONTEXT);
    #[no_mangle]
    fn MD5_write(ctx: *mut MD5_CONTEXT, inbuf: *const u8, inlen: u32);
    #[no_mangle]
    fn MD5_final(outbuf: *mut u8, ctx: *mut MD5_CONTEXT);
    #[no_mangle]
    fn SHA256_init(ctx: *mut SHA256_CONTEXT);
    #[no_mangle]
    fn SHA256_write(ctx: *mut SHA256_CONTEXT, inbuf: *const u8, inlen: u32);
    #[no_mangle]
    fn SHA256_final(outbuf: *mut u8, ctx: *mut SHA256_CONTEXT);
    #[no_mangle]
    fn SHA384_init(ctx: *mut SHA512_CONTEXT);
    #[no_mangle]
    fn SHA512_init(ctx: *mut SHA512_CONTEXT);
    #[no_mangle]
    fn SHA512_write(ctx: *mut SHA512_CONTEXT, inbuf: *const u8, inlen: u32);
    #[no_mangle]
    fn SHA512_final(outbuf: *mut u8, ctx: *mut SHA512_CONTEXT);
    #[no_mangle]
    fn ARC4(ctx: *mut ARC4_CONTEXT, len: u32, inbuf: *const u8, outbuf: *mut u8);
    #[no_mangle]
    fn ARC4_set_key(ctx: *mut ARC4_CONTEXT, keylen: u32, key: *const u8);
    #[no_mangle]
    fn AES_ecb_encrypt(
        key: *const u8,
        key_len: size_t,
        plain: *const u8,
        plain_len: size_t,
        cipher: *mut *mut u8,
        cipher_len: *mut size_t,
    );
    #[no_mangle]
    fn AES_cbc_encrypt_tectonic(
        key: *const u8,
        key_len: size_t,
        iv: *const u8,
        padding: i32,
        plain: *const u8,
        plain_len: size_t,
        cipher: *mut *mut u8,
        cipher_len: *mut size_t,
    );
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
    #[no_mangle]
    fn new(size: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn pdf_get_version() -> u32;
    /* Name does not include the / */
    /* pdf_add_dict requires key but pdf_add_array does not.
     * pdf_add_array always append elements to array.
     * They should be pdf_put_array(array, idx, element) and
     * pdf_put_dict(dict, key, value)
     */
    /* pdf_add_dict() want pdf_obj as key, however, key must always be name
     * object and pdf_lookup_dict() and pdf_remove_dict() uses const char as
     * key. This strange difference seems come from pdfdoc that first allocate
     * name objects frequently used (maybe 1000 times) such as /Type and does
     * pdf_link_obj() it rather than allocate/free-ing them each time. But I
     * already removed that.
     */
    #[no_mangle]
    fn get_unique_time_if_given() -> time_t;
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
    #[no_mangle]
    fn UC_is_valid(ucv: i32) -> bool;
    #[no_mangle]
    fn UC_UTF8_decode_char(pp: *mut *const u8, endptr: *const u8) -> i32;
    /* They just return PDF dictionary object.
     * Callers are completely responsible for doing right thing...
     */
    #[no_mangle]
    fn pdf_doc_get_dictionary(category: *const i8) -> *mut pdf_obj;
}
pub type __time_t = i64;
pub type size_t = u64;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5_CONTEXT {
    pub A: u32,
    pub B: u32,
    pub C: u32,
    pub D: u32,
    pub nblocks: size_t,
    pub buf: [u8; 64],
    pub count: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA256_CONTEXT {
    pub h0: u32,
    pub h1: u32,
    pub h2: u32,
    pub h3: u32,
    pub h4: u32,
    pub h5: u32,
    pub h6: u32,
    pub h7: u32,
    pub nblocks: size_t,
    pub buf: [u8; 64],
    pub count: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA512_STATE {
    pub h0: u64,
    pub h1: u64,
    pub h2: u64,
    pub h3: u64,
    pub h4: u64,
    pub h5: u64,
    pub h6: u64,
    pub h7: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA512_CONTEXT {
    pub state: SHA512_STATE,
    pub nblocks: size_t,
    pub buf: [u8; 128],
    pub count: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARC4_CONTEXT {
    pub idx_i: i32,
    pub idx_j: i32,
    pub sbox: [u8; 256],
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

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
/* Encryption support
 *
 * Supported: 40-128 bit RC4, 128 bit AES, 256 bit AES
 *
 * TODO: Convert password to PDFDocEncoding. SASLPrep stringpref for AESV3.
 */
/* PDF-2.0 is not published yet. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_sec {
    pub key: [u8; 32],
    pub key_size: i32,
    pub ID: [u8; 16],
    pub O: [u8; 48],
    pub U: [u8; 48],
    pub OE: [u8; 32],
    pub UE: [u8; 32],
    pub V: i32,
    pub R: i32,
    pub P: i32,
    pub setting: C2RustUnnamed_0,
    pub label: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub objnum: u64,
    pub gennum: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub use_aes: i32,
    pub encrypt_metadata: i32,
}
/* Dummy routine for stringprep - NOT IMPLEMENTED YET
 *
 * Preprocessing of a user-provided password consists first of
 * normalizing its representation by applying the "SASLPrep" profile (RFC 4013)
 * of the "stringprep" algorithm (RFC 3454) to the supplied password using the
 * Normalize and BiDi options.
 */
pub type Stringprep_profile_flags = i32;
static mut sec_data: pdf_sec = pdf_sec {
    key: [0; 32],
    key_size: 0,
    ID: [0; 16],
    O: [0; 48],
    U: [0; 48],
    OE: [0; 32],
    UE: [0; 32],
    V: 0,
    R: 0,
    P: 0,
    setting: C2RustUnnamed_0 {
        use_aes: 0,
        encrypt_metadata: 0,
    },
    label: C2RustUnnamed {
        objnum: 0,
        gennum: 0,
    },
};
static mut padding_bytes: [u8; 32] = [
    0x28, 0xbf, 0x4e, 0x5e, 0x4e, 0x75, 0x8a, 0x41, 0x64, 0, 0x4e, 0x56, 0xff, 0xfa, 0x1, 0x8,
    0x2e, 0x2e, 0, 0xb6, 0xd0, 0x68, 0x3e, 0x80, 0x2f, 0xc, 0xa9, 0xfe, 0x64, 0x53, 0x69, 0x7a,
];
static mut verbose: u8 = 0_u8;
/*

    This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

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
#[no_mangle]
pub unsafe extern "C" fn pdf_enc_set_verbose(mut level: i32) {
    verbose = level as u8; /* For AES IV */
}
unsafe extern "C" fn pdf_enc_init(mut use_aes: i32, mut encrypt_metadata: i32) {
    let mut current_time: time_t = 0;
    let mut p: *mut pdf_sec = &mut sec_data;
    current_time = get_unique_time_if_given();
    if current_time == -1i32 as time_t {
        current_time = time(0 as *mut time_t)
    }
    srand(current_time as u32);
    (*p).setting.use_aes = use_aes;
    (*p).setting.encrypt_metadata = encrypt_metadata;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_enc_compute_id_string(mut dviname: *const i8, mut pdfname: *const i8) {
    let mut p: *mut pdf_sec = &mut sec_data;
    let mut date_string: *mut i8 = 0 as *mut i8;
    let mut producer: *mut i8 = 0 as *mut i8;
    let mut current_time: time_t = 0;
    let mut bd_time: *mut tm = 0 as *mut tm;
    let mut md5: MD5_CONTEXT = MD5_CONTEXT {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    /* FIXME: This should be placed in main() or somewhere. */
    pdf_enc_init(1i32, 1i32);
    MD5_init(&mut md5);
    date_string = new((15_u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    current_time = get_unique_time_if_given();
    if current_time == -1i32 as time_t {
        time(&mut current_time);
        bd_time = localtime(&mut current_time)
    } else {
        bd_time = gmtime(&mut current_time)
    }
    sprintf(
        date_string,
        b"%04d%02d%02d%02d%02d%02d\x00" as *const u8 as *const i8,
        (*bd_time).tm_year + 1900i32,
        (*bd_time).tm_mon + 1i32,
        (*bd_time).tm_mday,
        (*bd_time).tm_hour,
        (*bd_time).tm_min,
        (*bd_time).tm_sec,
    );
    MD5_write(&mut md5, date_string as *mut u8, strlen(date_string) as u32);
    free(date_string as *mut libc::c_void);
    producer = new((strlen(
        b"%s-%s, Copyright 2002-2015 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata\x00"
            as *const u8 as *const i8,
    )
    .wrapping_add(strlen(b"xdvipdfmx\x00" as *const u8 as *const i8))
    .wrapping_add(strlen(b"0.1\x00" as *const u8 as *const i8)) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    sprintf(
        producer,
        b"%s-%s, Copyright 2002-2015 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata\x00"
            as *const u8 as *const i8,
        b"xdvipdfmx\x00" as *const u8 as *const i8,
        b"0.1\x00" as *const u8 as *const i8,
    );
    MD5_write(&mut md5, producer as *mut u8, strlen(producer) as u32);
    free(producer as *mut libc::c_void);
    if !dviname.is_null() {
        MD5_write(&mut md5, dviname as *const u8, strlen(dviname) as u32);
    }
    if !pdfname.is_null() {
        MD5_write(&mut md5, pdfname as *const u8, strlen(pdfname) as u32);
    }
    MD5_final((*p).ID.as_mut_ptr(), &mut md5);
}
unsafe extern "C" fn passwd_padding(mut src: *const i8, mut dst: *mut u8) {
    let mut len: i32 = 0;
    len = (if (32i32 as u64) < strlen(src) {
        32i32 as u64
    } else {
        strlen(src)
    }) as i32;
    memcpy(
        dst as *mut libc::c_void,
        src as *const libc::c_void,
        len as u64,
    );
    memcpy(
        dst.offset(len as isize) as *mut libc::c_void,
        padding_bytes.as_ptr() as *const libc::c_void,
        (32i32 - len) as u64,
    );
}
unsafe extern "C" fn compute_owner_password(
    mut p: *mut pdf_sec,
    mut opasswd: *const i8,
    mut upasswd: *const i8,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut padded: [u8; 32] = [0; 32];
    let mut md5: MD5_CONTEXT = MD5_CONTEXT {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    let mut arc4: ARC4_CONTEXT = ARC4_CONTEXT {
        idx_i: 0,
        idx_j: 0,
        sbox: [0; 256],
    };
    let mut hash: [u8; 32] = [0; 32];
    passwd_padding(
        if strlen(opasswd) > 0i32 as u64 {
            opasswd
        } else {
            upasswd
        },
        padded.as_mut_ptr(),
    );
    MD5_init(&mut md5);
    MD5_write(&mut md5, padded.as_mut_ptr(), 32_u32);
    MD5_final(hash.as_mut_ptr(), &mut md5);
    if (*p).R >= 3i32 {
        i = 0i32;
        while i < 50i32 {
            /*
             * NOTE: We truncate each MD5 hash as in the following step.
             *       Otherwise Adobe Reader won't decrypt the PDF file.
             */
            MD5_init(&mut md5);
            MD5_write(&mut md5, hash.as_mut_ptr(), (*p).key_size as u32);
            MD5_final(hash.as_mut_ptr(), &mut md5);
            i += 1
        }
    }
    ARC4_set_key(&mut arc4, (*p).key_size as u32, hash.as_mut_ptr());
    passwd_padding(upasswd, padded.as_mut_ptr());
    let mut tmp1: [u8; 32] = [0; 32];
    let mut tmp2: [u8; 32] = [0; 32];
    let mut key: [u8; 16] = [0; 16];
    ARC4(&mut arc4, 32_u32, padded.as_mut_ptr(), tmp1.as_mut_ptr());
    if (*p).R >= 3i32 {
        i = 1i32;
        while i <= 19i32 {
            memcpy(
                tmp2.as_mut_ptr() as *mut libc::c_void,
                tmp1.as_mut_ptr() as *const libc::c_void,
                32i32 as u64,
            );
            j = 0i32;
            while j < (*p).key_size {
                key[j as usize] = (hash[j as usize] as i32 ^ i) as u8;
                j += 1
            }
            ARC4_set_key(&mut arc4, (*p).key_size as u32, key.as_mut_ptr());
            ARC4(&mut arc4, 32_u32, tmp2.as_mut_ptr(), tmp1.as_mut_ptr());
            i += 1
        }
    }
    memcpy(
        (*p).O.as_mut_ptr() as *mut libc::c_void,
        hash.as_mut_ptr() as *const libc::c_void,
        32i32 as u64,
    );
}
unsafe extern "C" fn compute_encryption_key(mut p: *mut pdf_sec, mut passwd: *const i8) {
    let mut i: i32 = 0;
    let mut hash: [u8; 32] = [0; 32];
    let mut padded: [u8; 32] = [0; 32];
    let mut md5: MD5_CONTEXT = MD5_CONTEXT {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    passwd_padding(passwd, padded.as_mut_ptr());
    MD5_init(&mut md5);
    MD5_write(&mut md5, padded.as_mut_ptr(), 32_u32);
    MD5_write(&mut md5, (*p).O.as_mut_ptr(), 32_u32);
    let mut tmp: [u8; 4] = [0; 4];
    tmp[0] = ((*p).P as u8 as i32 & 0xffi32) as u8;
    tmp[1] = (((*p).P >> 8i32) as u8 as i32 & 0xffi32) as u8;
    tmp[2] = (((*p).P >> 16i32) as u8 as i32 & 0xffi32) as u8;
    tmp[3] = (((*p).P >> 24i32) as u8 as i32 & 0xffi32) as u8;
    MD5_write(&mut md5, tmp.as_mut_ptr(), 4_u32);
    MD5_write(&mut md5, (*p).ID.as_mut_ptr(), 16_u32);
    MD5_final(hash.as_mut_ptr(), &mut md5);
    if (*p).R >= 3i32 {
        i = 0i32;
        while i < 50i32 {
            /*
             * NOTE: We truncate each MD5 hash as in the following step.
             *       Otherwise Adobe Reader won't decrypt the PDF file.
             */
            MD5_init(&mut md5);
            MD5_write(&mut md5, hash.as_mut_ptr(), (*p).key_size as u32);
            MD5_final(hash.as_mut_ptr(), &mut md5);
            i += 1
        }
    }
    memcpy(
        (*p).key.as_mut_ptr() as *mut libc::c_void,
        hash.as_mut_ptr() as *const libc::c_void,
        (*p).key_size as u64,
    );
}
unsafe extern "C" fn compute_user_password(mut p: *mut pdf_sec, mut uplain: *const i8) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut arc4: ARC4_CONTEXT = ARC4_CONTEXT {
        idx_i: 0,
        idx_j: 0,
        sbox: [0; 256],
    };
    let mut md5: MD5_CONTEXT = MD5_CONTEXT {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    let mut upasswd: [u8; 32] = [0; 32];
    compute_encryption_key(p, uplain);
    match (*p).R {
        2 => {
            ARC4_set_key(&mut arc4, (*p).key_size as u32, (*p).key.as_mut_ptr());
            ARC4(
                &mut arc4,
                32_u32,
                padding_bytes.as_ptr(),
                upasswd.as_mut_ptr(),
            );
        }
        3 | 4 => {
            let mut hash: [u8; 32] = [0; 32];
            let mut tmp1: [u8; 32] = [0; 32];
            let mut tmp2: [u8; 32] = [0; 32];
            MD5_init(&mut md5);
            MD5_write(&mut md5, padding_bytes.as_ptr(), 32_u32);
            MD5_write(&mut md5, (*p).ID.as_mut_ptr(), 16_u32);
            MD5_final(hash.as_mut_ptr(), &mut md5);
            ARC4_set_key(&mut arc4, (*p).key_size as u32, (*p).key.as_mut_ptr());
            ARC4(&mut arc4, 16_u32, hash.as_mut_ptr(), tmp1.as_mut_ptr());
            i = 1i32;
            while i <= 19i32 {
                let mut key: [u8; 16] = [0; 16];
                memcpy(
                    tmp2.as_mut_ptr() as *mut libc::c_void,
                    tmp1.as_mut_ptr() as *const libc::c_void,
                    16i32 as u64,
                );
                j = 0i32;
                while j < (*p).key_size {
                    key[j as usize] = ((*p).key[j as usize] as i32 ^ i) as u8;
                    j += 1
                }
                ARC4_set_key(&mut arc4, (*p).key_size as u32, key.as_mut_ptr());
                ARC4(&mut arc4, 16_u32, tmp2.as_mut_ptr(), tmp1.as_mut_ptr());
                i += 1
            }
            memcpy(
                upasswd.as_mut_ptr() as *mut libc::c_void,
                tmp1.as_mut_ptr() as *const libc::c_void,
                32i32 as u64,
            );
        }
        _ => {
            panic!("Invalid revision number.");
        }
    }
    memcpy(
        (*p).U.as_mut_ptr() as *mut libc::c_void,
        upasswd.as_mut_ptr() as *const libc::c_void,
        32i32 as u64,
    );
}
/* Algorithm 2.B from ISO 32000-1 chapter 7 */
unsafe extern "C" fn compute_hash_V5(
    mut hash: *mut u8,
    mut passwd: *const i8,
    mut salt: *const u8,
    mut user_key: *const u8,
    mut R: i32,
)
/* revision */
{
    let mut sha: SHA256_CONTEXT = SHA256_CONTEXT {
        h0: 0,
        h1: 0,
        h2: 0,
        h3: 0,
        h4: 0,
        h5: 0,
        h6: 0,
        h7: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    let mut K: [u8; 64] = [0; 64];
    let mut K_len: size_t = 0;
    let mut nround: i32 = 0;
    SHA256_init(&mut sha);
    SHA256_write(&mut sha, passwd as *const u8, strlen(passwd) as u32);
    SHA256_write(&mut sha, salt, 8_u32);
    if !user_key.is_null() {
        SHA256_write(&mut sha, user_key, 48_u32);
    }
    SHA256_final(hash, &mut sha);
    assert!(R == 5i32 || R == 6i32);
    if R == 5i32 {
        return;
    }
    memcpy(
        K.as_mut_ptr() as *mut libc::c_void,
        hash as *const libc::c_void,
        32i32 as u64,
    );
    K_len = 32i32 as size_t;
    nround = 1i32;
    loop
    /* Initial K count as nround 0. */
    {
        let mut K1: [u8; 256] = [0; 256];
        let mut Kr: *mut u8 = 0 as *mut u8;
        let mut E: *mut u8 = 0 as *mut u8;
        let mut K1_len: size_t = 0;
        let mut E_len: size_t = 0;
        let mut i: i32 = 0;
        let mut c: i32 = 0;
        let mut E_mod3: i32 = 0i32;
        K1_len = strlen(passwd)
            .wrapping_add(K_len)
            .wrapping_add((if !user_key.is_null() { 48i32 } else { 0i32 }) as u64);
        assert!(K1_len < 240i32 as u64);
        memcpy(
            K1.as_mut_ptr() as *mut libc::c_void,
            passwd as *const libc::c_void,
            strlen(passwd),
        );
        memcpy(
            K1.as_mut_ptr().offset(strlen(passwd) as isize) as *mut libc::c_void,
            K.as_mut_ptr() as *const libc::c_void,
            K_len,
        );
        if !user_key.is_null() {
            memcpy(
                K1.as_mut_ptr()
                    .offset(strlen(passwd) as isize)
                    .offset(K_len as isize) as *mut libc::c_void,
                user_key as *const libc::c_void,
                48i32 as u64,
            );
        }
        Kr = new((K1_len.wrapping_mul(64i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32) as *mut u8;
        i = 0i32;
        while i < 64i32 {
            memcpy(
                Kr.offset((i as u64).wrapping_mul(K1_len) as isize) as *mut libc::c_void,
                K1.as_mut_ptr() as *const libc::c_void,
                K1_len,
            );
            i += 1
        }
        AES_cbc_encrypt_tectonic(
            K.as_mut_ptr(),
            16i32 as size_t,
            K.as_mut_ptr().offset(16),
            0i32,
            Kr,
            K1_len.wrapping_mul(64i32 as u64),
            &mut E,
            &mut E_len,
        );
        free(Kr as *mut libc::c_void);
        i = 0i32;
        while i < 16i32 {
            E_mod3 += *E.offset(i as isize) as i32;
            i += 1
        }
        E_mod3 %= 3i32;
        match E_mod3 {
            0 => {
                let mut sha_0: SHA256_CONTEXT = SHA256_CONTEXT {
                    h0: 0,
                    h1: 0,
                    h2: 0,
                    h3: 0,
                    h4: 0,
                    h5: 0,
                    h6: 0,
                    h7: 0,
                    nblocks: 0,
                    buf: [0; 64],
                    count: 0,
                };
                SHA256_init(&mut sha_0);
                SHA256_write(&mut sha_0, E, E_len as u32);
                SHA256_final(K.as_mut_ptr(), &mut sha_0);
                K_len = 32i32 as size_t
            }
            1 => {
                let mut sha_1: SHA512_CONTEXT = SHA512_CONTEXT {
                    state: SHA512_STATE {
                        h0: 0,
                        h1: 0,
                        h2: 0,
                        h3: 0,
                        h4: 0,
                        h5: 0,
                        h6: 0,
                        h7: 0,
                    },
                    nblocks: 0,
                    buf: [0; 128],
                    count: 0,
                };
                SHA384_init(&mut sha_1);
                SHA512_write(&mut sha_1, E, E_len as u32);
                SHA512_final(K.as_mut_ptr(), &mut sha_1);
                K_len = 48i32 as size_t
            }
            2 => {
                let mut sha_2: SHA512_CONTEXT = SHA512_CONTEXT {
                    state: SHA512_STATE {
                        h0: 0,
                        h1: 0,
                        h2: 0,
                        h3: 0,
                        h4: 0,
                        h5: 0,
                        h6: 0,
                        h7: 0,
                    },
                    nblocks: 0,
                    buf: [0; 128],
                    count: 0,
                };
                SHA512_init(&mut sha_2);
                SHA512_write(&mut sha_2, E, E_len as u32);
                SHA512_final(K.as_mut_ptr(), &mut sha_2);
                K_len = 64i32 as size_t
            }
            _ => {}
        }
        c = *E.offset(E_len.wrapping_sub(1i32 as u64) as isize) as i32;
        free(E as *mut libc::c_void);
        if nround >= 64i32 && c <= nround - 32i32 {
            break;
        }
        nround += 1
    }
    memcpy(
        hash as *mut libc::c_void,
        K.as_mut_ptr() as *const libc::c_void,
        32i32 as u64,
    );
}
unsafe extern "C" fn compute_owner_password_V5(mut p: *mut pdf_sec, mut oplain: *const i8) {
    let mut vsalt: [u8; 8] = [0; 8];
    let mut ksalt: [u8; 8] = [0; 8];
    let mut hash: [u8; 32] = [0; 32];
    let mut OE: *mut u8 = 0 as *mut u8;
    let mut iv: [u8; 16] = [0; 16];
    let mut OE_len: size_t = 0;
    let mut i: i32 = 0;
    i = 0i32;
    while i < 8i32 {
        vsalt[i as usize] = (rand() % 256i32) as u8;
        ksalt[i as usize] = (rand() % 256i32) as u8;
        i += 1
    }
    compute_hash_V5(
        hash.as_mut_ptr(),
        oplain,
        vsalt.as_mut_ptr(),
        (*p).U.as_mut_ptr(),
        (*p).R,
    );
    memcpy(
        (*p).O.as_mut_ptr() as *mut libc::c_void,
        hash.as_mut_ptr() as *const libc::c_void,
        32i32 as u64,
    );
    memcpy(
        (*p).O.as_mut_ptr().offset(32) as *mut libc::c_void,
        vsalt.as_mut_ptr() as *const libc::c_void,
        8i32 as u64,
    );
    memcpy(
        (*p).O.as_mut_ptr().offset(40) as *mut libc::c_void,
        ksalt.as_mut_ptr() as *const libc::c_void,
        8i32 as u64,
    );
    compute_hash_V5(
        hash.as_mut_ptr(),
        oplain,
        ksalt.as_mut_ptr(),
        (*p).U.as_mut_ptr(),
        (*p).R,
    );
    memset(iv.as_mut_ptr() as *mut libc::c_void, 0i32, 16i32 as u64);
    AES_cbc_encrypt_tectonic(
        hash.as_mut_ptr(),
        32i32 as size_t,
        iv.as_mut_ptr(),
        0i32,
        (*p).key.as_mut_ptr(),
        (*p).key_size as size_t,
        &mut OE,
        &mut OE_len,
    );
    memcpy(
        (*p).OE.as_mut_ptr() as *mut libc::c_void,
        OE as *const libc::c_void,
        32i32 as u64,
    );
    free(OE as *mut libc::c_void);
}
unsafe extern "C" fn compute_user_password_V5(mut p: *mut pdf_sec, mut uplain: *const i8) {
    let mut vsalt: [u8; 8] = [0; 8];
    let mut ksalt: [u8; 8] = [0; 8];
    let mut hash: [u8; 32] = [0; 32];
    let mut UE: *mut u8 = 0 as *mut u8;
    let mut iv: [u8; 16] = [0; 16];
    let mut UE_len: size_t = 0;
    let mut i: i32 = 0;
    i = 0i32;
    while i < 8i32 {
        vsalt[i as usize] = (rand() % 256i32) as u8;
        ksalt[i as usize] = (rand() % 256i32) as u8;
        i += 1
    }
    compute_hash_V5(
        hash.as_mut_ptr(),
        uplain,
        vsalt.as_mut_ptr(),
        0 as *const u8,
        (*p).R,
    );
    memcpy(
        (*p).U.as_mut_ptr() as *mut libc::c_void,
        hash.as_mut_ptr() as *const libc::c_void,
        32i32 as u64,
    );
    memcpy(
        (*p).U.as_mut_ptr().offset(32) as *mut libc::c_void,
        vsalt.as_mut_ptr() as *const libc::c_void,
        8i32 as u64,
    );
    memcpy(
        (*p).U.as_mut_ptr().offset(40) as *mut libc::c_void,
        ksalt.as_mut_ptr() as *const libc::c_void,
        8i32 as u64,
    );
    compute_hash_V5(
        hash.as_mut_ptr(),
        uplain,
        ksalt.as_mut_ptr(),
        0 as *const u8,
        (*p).R,
    );
    memset(iv.as_mut_ptr() as *mut libc::c_void, 0i32, 16i32 as u64);
    AES_cbc_encrypt_tectonic(
        hash.as_mut_ptr(),
        32i32 as size_t,
        iv.as_mut_ptr(),
        0i32,
        (*p).key.as_mut_ptr(),
        (*p).key_size as size_t,
        &mut UE,
        &mut UE_len,
    );
    memcpy(
        (*p).UE.as_mut_ptr() as *mut libc::c_void,
        UE as *const libc::c_void,
        32i32 as u64,
    );
    free(UE as *mut libc::c_void);
}
unsafe extern "C" fn check_version(mut p: *mut pdf_sec, mut version: i32) {
    if (*p).V > 2i32 && version < 4i32 {
        warn!("Current encryption setting requires PDF version >= 1.4.");
        (*p).V = 1i32;
        (*p).key_size = 5i32
    } else if (*p).V == 4i32 && version < 5i32 {
        warn!("Current encryption setting requires PDF version >= 1.5.");
        (*p).V = 2i32
    } else if (*p).V == 5i32 && version < 7i32 {
        warn!("Current encryption setting requires PDF version >= 1.7 (plus Adobe Extension Level 3).");
        (*p).V = 4i32
    };
}
unsafe extern "C" fn stringprep_profile(
    mut input: *const i8,
    mut output: *mut *mut i8,
    mut profile: *const i8,
    mut flags: Stringprep_profile_flags,
) -> i32 {
    let mut p: *const i8 = 0 as *const i8;
    let mut endptr: *const i8 = 0 as *const i8;
    p = input;
    endptr = p.offset(strlen(p) as isize);
    while p < endptr {
        let mut ucv: i32 = UC_UTF8_decode_char(
            &mut p as *mut *const i8 as *mut *const u8,
            endptr as *const u8,
        );
        if !UC_is_valid(ucv) {
            return -1i32;
        }
    }
    *output = new((strlen(input).wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    strcpy(*output, input);
    0i32
}
unsafe extern "C" fn preproc_password(
    mut passwd: *const i8,
    mut outbuf: *mut i8,
    mut V: i32,
) -> i32 {
    let mut saslpwd: *mut i8 = 0 as *mut i8;
    let mut error: i32 = 0i32;
    memset(outbuf as *mut libc::c_void, 0i32, 128i32 as u64);
    match V {
        1 | 2 | 3 | 4 => {
            let mut i: size_t = 0;
            /* Need to be converted to PDFDocEncoding - UNIMPLEMENTED */
            i = 0i32 as size_t;
            while i < strlen(passwd) {
                if (*passwd.offset(i as isize) as i32) < 0x20i32
                    || *passwd.offset(i as isize) as i32 > 0x7ei32
                {
                    warn!("Non-ASCII-printable character found in password.");
                }
                i = i.wrapping_add(1)
            }
            memcpy(
                outbuf as *mut libc::c_void,
                passwd as *const libc::c_void,
                if (127i32 as u64) < strlen(passwd) {
                    127i32 as u64
                } else {
                    strlen(passwd)
                },
            );
        }
        5 => {
            /* This is a dummy routine - not actually stringprep password... */
            if stringprep_profile(
                passwd,
                &mut saslpwd,
                b"SASLprep\x00" as *const u8 as *const i8,
                0i32,
            ) != 0i32
            {
                return -1i32;
            } else {
                if !saslpwd.is_null() {
                    memcpy(
                        outbuf as *mut libc::c_void,
                        saslpwd as *const libc::c_void,
                        if (127i32 as u64) < strlen(saslpwd) {
                            127i32 as u64
                        } else {
                            strlen(saslpwd)
                        },
                    );
                    free(saslpwd as *mut libc::c_void);
                }
            }
        }
        _ => error = -1i32,
    }
    error
}
#[no_mangle]
pub unsafe extern "C" fn pdf_enc_set_passwd(
    mut bits: u32,
    mut perm: u32,
    mut oplain: *const i8,
    mut uplain: *const i8,
) {
    let mut p: *mut pdf_sec = &mut sec_data;
    let mut opasswd: [i8; 128] = [0; 128];
    let mut upasswd: [i8; 128] = [0; 128];
    let mut version: i32 = 0;
    assert!(!oplain.is_null());
    assert!(!uplain.is_null());
    version = pdf_get_version() as i32;
    (*p).key_size = bits.wrapping_div(8_u32) as i32;
    if (*p).key_size == 5i32 {
        /* 40bit */
        (*p).V = 1i32
    } else if (*p).key_size <= 16i32 {
        (*p).V = if (*p).setting.use_aes != 0 {
            4i32
        } else {
            2i32
        }
    } else if (*p).key_size == 32i32 {
        (*p).V = 5i32
    } else {
        warn!("Key length {} unsupported.", bits);
        (*p).key_size = 5i32;
        (*p).V = 2i32
    }
    check_version(p, version);
    (*p).P = (perm | 0xc0u32) as i32;
    match (*p).V {
        1 => (*p).R = if ((*p).P as i64) < 0x100 { 2i32 } else { 3i32 },
        2 | 3 => (*p).R = 3i32,
        4 => (*p).R = 4i32,
        5 => (*p).R = 6i32,
        _ => (*p).R = 3i32,
    }
    memset(
        opasswd.as_mut_ptr() as *mut libc::c_void,
        0i32,
        128i32 as u64,
    );
    memset(
        upasswd.as_mut_ptr() as *mut libc::c_void,
        0i32,
        128i32 as u64,
    );
    /* Password must be preprocessed. */
    if preproc_password(oplain, opasswd.as_mut_ptr(), (*p).V) < 0i32 {
        warn!("Invaid UTF-8 string for password.");
    }
    if preproc_password(uplain, upasswd.as_mut_ptr(), (*p).V) < 0i32 {
        warn!("Invalid UTF-8 string for passowrd.");
    }
    if (*p).R >= 3i32 {
        (*p).P = ((*p).P as u32 | 0xfffff000u32) as i32
    }
    if (*p).V < 5i32 {
        compute_owner_password(p, opasswd.as_mut_ptr(), upasswd.as_mut_ptr());
        compute_user_password(p, upasswd.as_mut_ptr());
    } else if (*p).V == 5i32 {
        let mut i: i32 = 0;
        i = 0i32;
        while i < 32i32 {
            (*p).key[i as usize] = (rand() % 256i32) as u8;
            i += 1
        }
        (*p).key_size = 32i32;
        /* uses p->U */
        compute_user_password_V5(p, upasswd.as_mut_ptr());
        compute_owner_password_V5(p, opasswd.as_mut_ptr());
    };
}
unsafe extern "C" fn calculate_key(mut p: *mut pdf_sec, mut key: *mut u8) {
    let mut len: i32 = (*p).key_size + 5i32;
    let mut tmp: [u8; 25] = [0; 25];
    let mut md5: MD5_CONTEXT = MD5_CONTEXT {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    memcpy(
        tmp.as_mut_ptr() as *mut libc::c_void,
        (*p).key.as_mut_ptr() as *const libc::c_void,
        (*p).key_size as u64,
    );
    tmp[(*p).key_size as usize] = ((*p).label.objnum as u8 as i32 & 0xffi32) as u8;
    tmp[((*p).key_size + 1i32) as usize] =
        (((*p).label.objnum >> 8i32) as u8 as i32 & 0xffi32) as u8;
    tmp[((*p).key_size + 2i32) as usize] =
        (((*p).label.objnum >> 16i32) as u8 as i32 & 0xffi32) as u8;
    tmp[((*p).key_size + 3i32) as usize] = ((*p).label.gennum as u8 as i32 & 0xffi32) as u8;
    tmp[((*p).key_size + 4i32) as usize] =
        (((*p).label.gennum as i32 >> 8i32) as u8 as i32 & 0xffi32) as u8;
    if (*p).V >= 4i32 {
        tmp[((*p).key_size + 5i32) as usize] = 0x73_u8;
        tmp[((*p).key_size + 6i32) as usize] = 0x41_u8;
        tmp[((*p).key_size + 7i32) as usize] = 0x6c_u8;
        tmp[((*p).key_size + 8i32) as usize] = 0x54_u8;
        len += 4i32
    }
    MD5_init(&mut md5);
    MD5_write(&mut md5, tmp.as_mut_ptr(), len as u32);
    MD5_final(key, &mut md5);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_encrypt_data(
    mut plain: *const u8,
    mut plain_len: size_t,
    mut cipher: *mut *mut u8,
    mut cipher_len: *mut size_t,
) {
    let mut p: *mut pdf_sec = &mut sec_data;
    let mut key: [u8; 32] = [0; 32];
    match (*p).V {
        1 | 2 => {
            calculate_key(p, key.as_mut_ptr());
            let mut arc4: ARC4_CONTEXT = ARC4_CONTEXT {
                idx_i: 0,
                idx_j: 0,
                sbox: [0; 256],
            };
            *cipher_len = plain_len;
            *cipher = new(
                (*cipher_len as u32 as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32
            ) as *mut u8;
            ARC4_set_key(
                &mut arc4,
                (if 16i32 < (*p).key_size + 5i32 {
                    16i32
                } else {
                    (*p).key_size + 5i32
                }) as u32,
                key.as_mut_ptr(),
            );
            ARC4(&mut arc4, plain_len as u32, plain, *cipher);
        }
        4 => {
            calculate_key(p, key.as_mut_ptr());
            AES_cbc_encrypt_tectonic(
                key.as_mut_ptr(),
                (if 16i32 < (*p).key_size + 5i32 {
                    16i32
                } else {
                    (*p).key_size + 5i32
                }) as size_t,
                0 as *const u8,
                1i32,
                plain,
                plain_len,
                cipher,
                cipher_len,
            );
        }
        5 => {
            AES_cbc_encrypt_tectonic(
                (*p).key.as_mut_ptr(),
                (*p).key_size as size_t,
                0 as *const u8,
                1i32,
                plain,
                plain_len,
                cipher,
                cipher_len,
            );
        }
        _ => {
            panic!("pdfencrypt: Unexpected V value: {}", (*p).V);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_encrypt_obj() -> *mut pdf_obj {
    let mut p: *mut pdf_sec = &mut sec_data;
    let mut doc_encrypt: *mut pdf_obj = 0 as *mut pdf_obj;
    doc_encrypt = pdf_new_dict();
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"Filter\x00" as *const u8 as *const i8),
        pdf_new_name(b"Standard\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"V\x00" as *const u8 as *const i8),
        pdf_new_number((*p).V as f64),
    );
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"Length\x00" as *const u8 as *const i8),
        pdf_new_number(((*p).key_size * 8i32) as f64),
    );
    if (*p).V >= 4i32 {
        let mut CF: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut StdCF: *mut pdf_obj = 0 as *mut pdf_obj;
        CF = pdf_new_dict();
        StdCF = pdf_new_dict();
        pdf_add_dict(
            StdCF,
            pdf_new_name(b"CFM\x00" as *const u8 as *const i8),
            pdf_new_name(if (*p).V == 4i32 {
                b"AESV2\x00" as *const u8 as *const i8
            } else {
                b"AESV3\x00" as *const u8 as *const i8
            }),
        );
        pdf_add_dict(
            StdCF,
            pdf_new_name(b"AuthEvent\x00" as *const u8 as *const i8),
            pdf_new_name(b"DocOpen\x00" as *const u8 as *const i8),
        );
        pdf_add_dict(
            StdCF,
            pdf_new_name(b"Length\x00" as *const u8 as *const i8),
            pdf_new_number((*p).key_size as f64),
        );
        pdf_add_dict(
            CF,
            pdf_new_name(b"StdCF\x00" as *const u8 as *const i8),
            StdCF,
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"CF\x00" as *const u8 as *const i8),
            CF,
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"StmF\x00" as *const u8 as *const i8),
            pdf_new_name(b"StdCF\x00" as *const u8 as *const i8),
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"StrF\x00" as *const u8 as *const i8),
            pdf_new_name(b"StdCF\x00" as *const u8 as *const i8),
        );
    }
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"R\x00" as *const u8 as *const i8),
        pdf_new_number((*p).R as f64),
    );
    if (*p).V < 5i32 {
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"O\x00" as *const u8 as *const i8),
            pdf_new_string((*p).O.as_mut_ptr() as *const libc::c_void, 32i32 as size_t),
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"U\x00" as *const u8 as *const i8),
            pdf_new_string((*p).U.as_mut_ptr() as *const libc::c_void, 32i32 as size_t),
        );
    } else if (*p).V == 5i32 {
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"O\x00" as *const u8 as *const i8),
            pdf_new_string((*p).O.as_mut_ptr() as *const libc::c_void, 48i32 as size_t),
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"U\x00" as *const u8 as *const i8),
            pdf_new_string((*p).U.as_mut_ptr() as *const libc::c_void, 48i32 as size_t),
        );
    }
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"P\x00" as *const u8 as *const i8),
        pdf_new_number((*p).P as f64),
    );
    if (*p).V == 5i32 {
        let mut perms: [u8; 16] = [0; 16];
        let mut cipher: *mut u8 = 0 as *mut u8;
        let mut cipher_len: size_t = 0i32 as size_t;
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"OE\x00" as *const u8 as *const i8),
            pdf_new_string((*p).OE.as_mut_ptr() as *const libc::c_void, 32i32 as size_t),
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"UE\x00" as *const u8 as *const i8),
            pdf_new_string((*p).UE.as_mut_ptr() as *const libc::c_void, 32i32 as size_t),
        );
        perms[0] = ((*p).P & 0xffi32) as u8;
        perms[1] = ((*p).P >> 8i32 & 0xffi32) as u8;
        perms[2] = ((*p).P >> 16i32 & 0xffi32) as u8;
        perms[3] = ((*p).P >> 24i32 & 0xffi32) as u8;
        perms[4] = 0xff_u8;
        perms[5] = 0xff_u8;
        perms[6] = 0xff_u8;
        perms[7] = 0xff_u8;
        perms[8] = (if (*p).setting.encrypt_metadata != 0 {
            'T' as i32
        } else {
            'F' as i32
        }) as u8;
        perms[9] = 'a' as i32 as u8;
        perms[10] = 'd' as i32 as u8;
        perms[11] = 'b' as i32 as u8;
        perms[12] = 0_u8;
        perms[13] = 0_u8;
        perms[14] = 0_u8;
        perms[15] = 0_u8;
        AES_ecb_encrypt(
            (*p).key.as_mut_ptr(),
            (*p).key_size as size_t,
            perms.as_mut_ptr(),
            16i32 as size_t,
            &mut cipher,
            &mut cipher_len,
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"Perms\x00" as *const u8 as *const i8),
            pdf_new_string(cipher as *const libc::c_void, cipher_len),
        );
        free(cipher as *mut libc::c_void);
    }
    if (*p).R > 5i32 {
        let mut catalog: *mut pdf_obj =
            pdf_doc_get_dictionary(b"Catalog\x00" as *const u8 as *const i8);
        let mut ext: *mut pdf_obj = pdf_new_dict();
        let mut adbe: *mut pdf_obj = pdf_new_dict();
        pdf_add_dict(
            adbe,
            pdf_new_name(b"BaseVersion\x00" as *const u8 as *const i8),
            pdf_new_name(b"1.7\x00" as *const u8 as *const i8),
        );
        pdf_add_dict(
            adbe,
            pdf_new_name(b"ExtensionLevel\x00" as *const u8 as *const i8),
            pdf_new_number((if (*p).R == 5i32 { 3i32 } else { 8i32 }) as f64),
        );
        pdf_add_dict(
            ext,
            pdf_new_name(b"ADBE\x00" as *const u8 as *const i8),
            adbe,
        );
        pdf_add_dict(
            catalog,
            pdf_new_name(b"Extensions\x00" as *const u8 as *const i8),
            ext,
        );
    }
    doc_encrypt
}
#[no_mangle]
pub unsafe extern "C" fn pdf_enc_id_array() -> *mut pdf_obj {
    let mut p: *mut pdf_sec = &mut sec_data;
    let mut id: *mut pdf_obj = pdf_new_array();
    pdf_add_array(
        id,
        pdf_new_string((*p).ID.as_mut_ptr() as *const libc::c_void, 16i32 as size_t),
    );
    pdf_add_array(
        id,
        pdf_new_string((*p).ID.as_mut_ptr() as *const libc::c_void, 16i32 as size_t),
    );
    id
}
#[no_mangle]
pub unsafe extern "C" fn pdf_enc_set_label(mut label: u32) {
    let mut p: *mut pdf_sec = &mut sec_data;
    (*p).label.objnum = label as u64;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_enc_set_generation(mut generation: u32) {
    let mut p: *mut pdf_sec = &mut sec_data;
    (*p).label.gennum = generation as u16;
}
/* Order is important here */
