// This is free and unencumbered software released into the public domain.

use crate::sysexits::Sysexits;
use protoflow_blocks::{CoreBlocks, DelayType, IoBlocks, SysBlocks};
use protoflow_core::{SystemBuilding, SystemExecution};
use std::{path::PathBuf, time::Duration};

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
                let const_source = s.const_string(value);
                let line_encoder = s.encode_lines();
                let stdout = s.write_stdout();
                s.connect(&const_source.output, &line_encoder.input);
                s.connect(&line_encoder.output, &stdout.input);
            })
        }
        "Count" => System::build(|s| {
            let stdin = s.read_stdin();
            let line_decoder = s.decode_lines::<String>(); // TODO
            let counter = s.count();
            let line_encoder = s.encode_lines();
            let stdout = s.write_stdout();
            s.connect(&stdin.output, &line_decoder.input);
            s.connect(&line_decoder.output, &counter.input);
            s.connect(&counter.count, &line_encoder.input);
            s.connect(&line_encoder.output, &stdout.input);
        }),
        "Delay" => {
            let fixed_delay = params
                .iter()
                .find(|(k, _)| k == "fixed")
                .map(|(_, v)| v.as_str().parse::<f64>());
            if let Some(Err(_)) = fixed_delay {
                return Err(ExecuteError::InvalidParameter("fixed"))?;
            }
            let fixed_delay = fixed_delay.map(Result::unwrap);
            let delay = DelayType::Fixed(Duration::from_secs_f64(fixed_delay.unwrap()));
            System::build(|s| {
                let random_source = s.random::<u64>(None);
                let delayer = s.delay_by(delay);
                let line_encoder = s.encode_lines::<u64>();
                let stdout = s.write_stdout();
                s.connect(&random_source.output, &delayer.input);
                s.connect(&delayer.output, &line_encoder.input);
                s.connect(&line_encoder.output, &stdout.input);
            })
        }
        "Drop" => System::build(|s| {
            let stdin = s.read_stdin();
            let line_decoder = s.decode_lines::<String>(); // TODO
            let dropper = s.drop();
            s.connect(&stdin.output, &line_decoder.input);
            s.connect(&line_decoder.output, &dropper.input);
        }),
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
                let random_source = s.random::<u64>(seed);
                let line_encoder = s.encode_lines::<u64>();
                let stdout = s.write_stdout();
                s.connect(&random_source.output, &line_encoder.input);
                s.connect(&line_encoder.output, &stdout.input);
            })
        }
        "ReadEnv" => {
            let Some(name) = params.iter().find(|(k, _)| k == "name").map(|(_, v)| v) else {
                return Err(ExecuteError::MissingParameter("name"))?;
            };
            let name = name.clone();
            System::build(|s| {
                let name_param = s.const_string(name);
                let env_reader = s.read_env();
                let line_encoder = s.encode_lines();
                let stdout = s.write_stdout();
                s.connect(&name_param.output, &env_reader.name);
                s.connect(&env_reader.output, &line_encoder.input);
                s.connect(&line_encoder.output, &stdout.input);
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
                let path_param = s.const_string(path);
                let dir_reader = s.read_dir();
                let line_encoder = s.encode_lines();
                let stdout = s.write_stdout();
                s.connect(&path_param.output, &dir_reader.path);
                s.connect(&dir_reader.output, &line_encoder.input);
                s.connect(&line_encoder.output, &stdout.input);
            })
        }
        "ReadStdin" | "WriteStdout" => System::build(|s| {
            let stdin = s.read_stdin();
            let stdout = s.write_stdout();
            s.connect(&stdin.output, &stdout.input);
        }),
        "WriteStderr" => System::build(|s| {
            let stdin = s.read_stdin();
            let stderr = s.write_stderr();
            s.connect(&stdin.output, &stderr.input);
        }),
        _ => return Err(ExecuteError::UnknownSystem(path_or_uri.to_string()))?,
    };
    system.execute().unwrap().join().unwrap(); // TODO
    Ok(())
}
