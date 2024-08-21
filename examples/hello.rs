#!/usr/bin/env rust-script
//! This is free and unencumbered software released into the public domain.
//!
//! ```cargo
//! [dependencies]
//! protoflow = "0.2"
//! ```

use protoflow::{blocks::*, BlockResult};

fn main() -> BlockResult {
    System::run(|s| {
        let greeting = s.const_string("Hello, world!");

        let line_encoder = s.encode_lines();
        s.connect(&greeting.output, &line_encoder.input);

        let stdout = s.write_stdout();
        s.connect(&line_encoder.output, &stdout.input);
    })
}
