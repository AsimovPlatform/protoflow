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
        let config = StdioConfig {
            encoding: Default::default(),
            params: Default::default(),
        };

        let random_int = s.random_int();
        let number_encoder = s.encode_with::<i64>(config.encoding);
        let stdout = s.write_stdout();
        s.connect(&random_int.output, &number_encoder.input);

        s.connect(&number_encoder.output, &stdout.input);

        // // TODO use Split block
        // let path_const = s.const_string("random_log.txt");
        // let write_file = s.write_file();
        // s.connect(&path_const.output, &write_file.path);
        // s.connect(&number_encoder.output, &write_file.input);
    })
}
