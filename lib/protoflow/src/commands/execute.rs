// This is free and unencumbered software released into the public domain.

use crate::sysexits::Sysexits;
use protoflow_core::prelude::Bytes;
use std::path::PathBuf;

type System = protoflow_core::System<protoflow_core::transports::MpscTransport>;

#[derive(Debug)]
pub enum ExecuteError {
    MissingParameter(&'static str),
    InvalidParameter(&'static str),
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
                let r#const = s.block(Const::with_params(s.output(), value));
                let stdout = s.block(WriteStdout::new(s.input()));
                s.connect(&r#const.output, &stdout.input);
            })
        }
        "Random" => {
            use protoflow_blocks::{Random, Write, WriteStdout};
            let seed = params
                .iter()
                .find(|(k, _)| k == "seed")
                .map(|(_, v)| v.as_str().parse::<u64>());
            if let Some(Err(_)) = seed {
                return Err(ExecuteError::InvalidParameter("seed"))?;
            }
            let seed = seed.map(Result::unwrap);
            System::build(|s| {
                let random = s.block(Random::<u64>::with_params(s.output(), seed));
                let adapter = s.block(Write::<u64>::new(s.input(), s.output()));
                let stdout = s.block(WriteStdout::new(s.input()));
                s.connect(&random.output, &adapter.input);
                s.connect(&adapter.output, &stdout.input);
            })
        }
        "ReadEnv" => {
            use protoflow_blocks::{Const, ReadEnv, Write, WriteStdout};
            let Some(name) = params.iter().find(|(k, _)| k == "name").map(|(_, v)| v) else {
                return Err(ExecuteError::MissingParameter("name"))?;
            };
            let name = name.clone();
            System::build(|s| {
                let name = s.block(Const::<String>::with_params(s.output(), name));
                let env = s.block(ReadEnv::<String>::new(s.input(), s.output()));
                let adapter = s.block(Write::<String>::new(s.input(), s.output()));
                let stdout = s.block(WriteStdout::new(s.input()));
                s.connect(&name.output, &env.name);
                s.connect(&env.output, &adapter.input);
                s.connect(&adapter.output, &stdout.input);
            })
        }
        _ => return Err(ExecuteError::UnknownSystem(path_or_uri.to_string()))?,
    };
    system.execute().unwrap().join().unwrap(); // TODO
    Ok(())
}
