// This is free and unencumbered software released into the public domain.

use crate::sysexits::Sysexits;
use protoflow_blocks::{CoreBlocks, Encoding, IoBlocks, SysBlocks};
use protoflow_core::{SystemBuilding, SystemExecution};
use std::path::PathBuf;

type System = protoflow_blocks::System;

#[derive(Debug)]
pub enum ExecuteError {
    MissingParameter(&'static str),
    InvalidParameter(&'static str),
    UnknownSystem(String),
}

pub fn execute(block: &PathBuf, params: &Vec<(String, String)>) -> Result<(), Sysexits> {
    let path_or_uri = block.to_string_lossy().to_string();
    let system = match path_or_uri.as_ref() {
        "Buffer" => todo!(),
        "Const" => {
            let Some(value) = params
                .iter()
                .find(|(k, _)| k == "value")
                .map(|(_, v)| v.clone())
            else {
                return Err(ExecuteError::MissingParameter("value"))?;
            };
            System::build(|s| {
                let value = s.const_string(value);
                let encoder = s.encode_with(Encoding::TextWithNewlineSuffix);
                let stdout = s.write_stdout();
                s.connect(&value.output, &encoder.input);
                s.connect(&encoder.output, &stdout.input);
            })
        }
        "Count" => todo!(),
        "Delay" => todo!(),
        "Drop" => todo!(),
        "Random" => {
            let seed = params
                .iter()
                .find(|(k, _)| k == "seed")
                .map(|(_, v)| v.as_str().parse::<u64>());
            if let Some(Err(_)) = seed {
                return Err(ExecuteError::InvalidParameter("seed"))?;
            }
            let seed = seed.map(Result::unwrap);
            System::build(|s| {
                let random = s.random::<u64>(seed);
                let encoder = s.encode_with::<u64>(Encoding::TextWithNewlineSuffix);
                let stdout = s.write_stdout();
                s.connect(&random.output, &encoder.input);
                s.connect(&encoder.output, &stdout.input);
            })
        }
        "ReadEnv" => {
            let Some(name) = params.iter().find(|(k, _)| k == "name").map(|(_, v)| v) else {
                return Err(ExecuteError::MissingParameter("name"))?;
            };
            let name = name.clone();
            System::build(|s| {
                let name = s.const_string(name);
                let env = s.read_env();
                let encoder = s.encode_with(Encoding::TextWithNewlineSuffix);
                let stdout = s.write_stdout();
                s.connect(&name.output, &env.name);
                s.connect(&env.output, &encoder.input);
                s.connect(&encoder.output, &stdout.input);
            })
        }
        "ReadDir" => {
            let Some(path) = params
                .iter()
                .find(|(k, _)| k == "path")
                .map(|(_, v)| v.clone())
            else {
                return Err(ExecuteError::MissingParameter("path"))?;
            };
            System::build(|s| {
                let path = s.const_string(path);
                let dir = s.read_dir();
                let encoder = s.encode_with(Encoding::TextWithNewlineSuffix);
                let stdout = s.write_stdout();
                s.connect(&path.output, &dir.path);
                s.connect(&dir.output, &encoder.input);
                s.connect(&encoder.output, &stdout.input);
            })
        }
        _ => return Err(ExecuteError::UnknownSystem(path_or_uri.to_string()))?,
    };
    system.execute().unwrap().join().unwrap(); // TODO
    Ok(())
}
