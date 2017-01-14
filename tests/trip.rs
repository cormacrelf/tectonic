// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

// Our incarnation of the classic TRIP test. Unfortunately, the test is
// defined in terms of the precise terminal output and error handling behavior
// of the engine, so you can't do anything to improve the (incredibly poor) UX
// of the TeX engine without having to fudge what "the TRIP test" is. That is
// what we have done.
//
// Cargo tries to run tests in multiple simultaneous threads, which of course
// totally fails for Tectonic since the engine has tons of global state. The
// multithreading can be disabled by setting the RUST_TEST_THREADS environment
// variable to "1", but that's an annoying solution. So, we use a global mutex
// to achieve the same effect. Classy.

#[macro_use]
extern crate lazy_static;
extern crate tectonic;

use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Mutex;

use tectonic::engines::tex::OutputFormat;
use tectonic::io::{IOProvider, IOStack, MemoryIO};
use tectonic::io::testing::SingleInputFileIO;
use tectonic::TexEngine;

const TOP: &'static str = env!("CARGO_MANIFEST_DIR");


lazy_static! {
    static ref LOCK: Mutex<u8> = Mutex::new(0u8);
}

struct ExpectedInfo {
    name: OsString,
    contents: Vec<u8>
}

impl ExpectedInfo {
    pub fn read(pbase: &mut PathBuf, extension: &str) -> ExpectedInfo {
        pbase.set_extension(extension);
        let name = pbase.file_name().unwrap().to_owned();

        let mut f = File::open(pbase).unwrap();
        let mut contents = Vec::new();
        f.read_to_end(&mut contents).unwrap();

        ExpectedInfo { name: name, contents: contents }
    }

    pub fn test_data(&self, observed: &Vec<u8>) {
        if &self.contents == observed {
            return;
        }

        // For nontrivial tests, it's really tough to figure out what
        // changed without being able to do diffs, etc. So, write out the
        // buffers.
        {
            let mut n = self.name.clone();
            n.push(".expected");
            let mut f = File::create(n).unwrap();
            f.write_all(&self.contents).unwrap();
        }
        {
            let mut n = self.name.clone();
            n.push(".observed");
            let mut f = File::create(n).unwrap();
            f.write_all(observed).unwrap();
        }
        panic!("difference in {}; contents saved to disk", self.name.to_string_lossy());
    }

    pub fn test(&self, files: &HashMap<OsString, Vec<u8>>) {
        self.test_data(files.get(&self.name).unwrap());
    }
}


#[test]
fn trip_test() {
    let _guard = LOCK.lock().unwrap(); // until we're thread-safe ...

    let mut p = PathBuf::from(TOP);
    p.push("tests");
    p.push("trip");

    // An IOProvider for the format file.
    let mut fmt_path = p.clone();
    fmt_path.push("trip.fmt");
    let mut fmt = SingleInputFileIO::new(&fmt_path);

    // Ditto for the input file.
    p.push("trip");
    p.set_extension("tex");
    let mut tex = SingleInputFileIO::new(&p);

    // And the TFM file.
    p.set_extension("tfm");
    let mut tfm = SingleInputFileIO::new(&p);

    // Read in the expected outputs.
    let expected_log = ExpectedInfo::read(&mut p, "log");
    let expected_xdv = ExpectedInfo::read(&mut p, "xdv");
    let expected_fot = ExpectedInfo::read(&mut p, "fot");
    p.set_file_name("tripos");
    let expected_os = ExpectedInfo::read(&mut p, "tex");

    // MemoryIO layer that will accept the outputs. Save `files` since the
    // engine consumes `mem`.
    let mut mem = MemoryIO::new(true);

    // Run the engine!
    {
        let mut io = IOStack::new(vec![
            &mut mem as &mut IOProvider,
            &mut tex,
            &mut fmt,
            &mut tfm,
        ]);
        let mut e = TexEngine::new ();
        e.set_halt_on_error_mode (false);
        e.set_output_format (OutputFormat::Xdv);
        e.process(&mut io, "trip.fmt", "trip").unwrap();
    }

    // Check that outputs match expectations.
    let files = &*mem.files.borrow();
    expected_log.test(files);
    expected_xdv.test(files);
    expected_os.test(files);
    expected_fot.test_data(files.get(OsStr::new("")).unwrap());
}


#[test]
fn etrip_test() {
    let _guard = LOCK.lock().unwrap(); // until we're thread-safe ...

    let mut p = PathBuf::from(TOP);
    p.push("tests");
    p.push("trip");

    // An IOProvider for the format file.
    let mut fmt_path = p.clone();
    fmt_path.push("etrip.fmt");
    let mut fmt = SingleInputFileIO::new(&fmt_path);

    // Ditto for the input file.
    p.push("etrip");
    p.set_extension("tex");
    let mut tex = SingleInputFileIO::new(&p);

    // And the TFM file.
    p.set_extension("tfm");
    let mut tfm = SingleInputFileIO::new(&p);

    // Read in the expected outputs.
    let expected_log = ExpectedInfo::read(&mut p, "log");
    let expected_xdv = ExpectedInfo::read(&mut p, "xdv");
    let expected_fot = ExpectedInfo::read(&mut p, "fot");
    let expected_out = ExpectedInfo::read(&mut p, "out");

    // MemoryIO layer that will accept the outputs. Save `files` since the
    // engine consumes `mem`.
    let mut mem = MemoryIO::new(true);
    let files = mem.files.clone();

    // Run the engine!
    {
        let mut io = IOStack::new(vec![
            &mut mem,
            &mut tex,
            &mut fmt,
            &mut tfm,
        ]);
        let mut e = TexEngine::new ();
        e.set_halt_on_error_mode (false);
        e.set_output_format (OutputFormat::Xdv);
        e.process(&mut io, "etrip.fmt", "etrip").unwrap();
    }

    // Check that outputs match expectations.
    let files = &*files.borrow();
    expected_log.test(files);
    expected_xdv.test(files);
    expected_out.test(files);
    expected_fot.test_data(files.get(OsStr::new("")).unwrap());
}