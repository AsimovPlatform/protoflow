#!/usr/bin/env rust-script
//! This is free and unencumbered software released into the public domain.
//!
//! ```cargo
//! [dependencies]
//! protoflow = "0.4.3"
//! ```

use protoflow::{blocks::*, BlockResult};

fn main() -> BlockResult {
    System::run(|s| {
        let file_path = s.const_string("log_directories.txt");
        let reader = s.read_file();
        let stdout = s.write_stdout();
        s.connect(&file_path.output, &reader.path);
        s.connect(&reader.output, &stdout.input);
    })
}