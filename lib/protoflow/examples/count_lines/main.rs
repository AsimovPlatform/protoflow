// This is free and unencumbered software released into the public domain.

use protoflow::{blocks::*, BlockResult};

pub fn main() -> BlockResult {
    System::run(|s| {
        let stdin = s.read_stdin();

        let line_decoder = s.decode_lines();
        s.connect(&stdin.output, &line_decoder.input);

        let counter = s.count::<String>();
        s.connect(&line_decoder.output, &counter.input);

        let count_encoder = s.encode_lines();
        s.connect(&counter.count, &count_encoder.input);

        let stdout = s.write_stdout();
        s.connect(&count_encoder.output, &stdout.input);
    })
}
