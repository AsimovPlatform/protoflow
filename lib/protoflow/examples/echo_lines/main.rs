// This is free and unencumbered software released into the public domain.

use protoflow::{blocks::*, BlockResult};

pub fn main() -> BlockResult {
    System::run(|s| {
        let stdin = s.read_stdin();
        let stdout = s.write_stdout();
        s.connect(&stdin.output, &stdout.input);
    })
}
