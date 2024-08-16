// This is free and unencumbered software released into the public domain.

use crate::sysexits::Sysexits;
use protoflow_core::prelude::Bytes;
use std::path::PathBuf;

type System = protoflow_core::System<protoflow_core::transports::MpscTransport>;

#[derive(Debug)]
pub enum ExecuteError {
    MissingParameter(&'static str),
    UnknownSystem(String),
}

pub fn execute(block: &PathBuf, params: &Vec<(String, String)>) -> Result<(), Sysexits> {
    let path_or_uri = block.to_string_lossy();
    let system = match path_or_uri.as_ref() {
        "Const" => {
            use protoflow_blocks::{Const, WriteStdout};
            let Some(value) = params.iter().find(|(k, _)| k == "value").map(|(_, v)| v) else {
                return Err(ExecuteError::MissingParameter("value"))?;
            };
            let value = Bytes::from(value.clone());
            System::build(|s| {
                let stdout = s.block(WriteStdout::new(s.input()));
                let source = s.block(Const::with_params(s.output(), value));
                s.connect(&source.output, &stdout.input);
            })
        }
        _ => return Err(ExecuteError::UnknownSystem(path_or_uri.to_string()))?,
    };
    system.execute().unwrap().join().unwrap(); // TODO
    Ok(())
}
