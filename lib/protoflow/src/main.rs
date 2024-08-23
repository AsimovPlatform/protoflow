// This is free and unencumbered software released into the public domain.

#![deny(unsafe_code)]
#![allow(unused)]

mod commands {
    #[cfg(feature = "beta")]
    pub mod check;
    #[cfg(feature = "beta")]
    pub mod config;
    pub mod execute;
    #[cfg(feature = "beta")]
    pub mod generate;
}
use commands::*;

mod exit;

use crate::exit::ExitCode;
use clientele::{
    crates::clap::{Args, Parser, Subcommand},
    StandardOptions,
};
use protoflow_blocks::Encoding;
use std::{error::Error, path::PathBuf, str::FromStr};

/// Protoflow Command-Line Interface (CLI)
#[derive(Debug, Parser)]
#[command(name = "Protoflow")]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Show the current configuration
    #[cfg(feature = "beta")]
    Config {},

    /// Check the syntax of a Protoflow system
    #[cfg(feature = "beta")]
    Check {
        /// Pathnames of Protoflow files to check
        #[clap(default_value = "/dev/stdin")]
        paths: Vec<PathBuf>,
    },

    /// Execute a Protoflow system or block
    Execute {
        /// Pathname of the Protoflow system or block
        block: PathBuf,

        /// Specify the message encoding to use on stdin/stdout
        #[clap(short = 'e', long, value_parser = parse_encoding, default_value = "text")]
        encoding: Encoding,

        /// Specify block parameters in key=value format
        #[clap(value_parser = parse_kv_param::<String, String>)]
        params: Vec<(String, String)>,
    },

    /// Generate code from a Protoflow system
    #[cfg(feature = "beta")]
    Generate {
        /// Pathname of the Protoflow file
        path: PathBuf,
    },
}

pub fn main() -> Result<(), ExitCode> {
    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    if options.flags.version {
        println!("protoflow {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if options.flags.license {
        println!("This is free and unencumbered software released into the public domain.");
        return Ok(());
    }

    // Configure verbose/debug output:
    if options.flags.verbose > 0 || options.flags.debug {
        // TODO: configure tracing
    }

    match options.command {
        #[cfg(feature = "beta")]
        Command::Config {} => config::config(),
        #[cfg(feature = "beta")]
        Command::Check { paths } => check::check(paths),
        Command::Execute {
            block,
            encoding,
            params,
        } => execute::execute(block, params, encoding),
        #[cfg(feature = "beta")]
        Command::Generate { path } => generate::generate(path),
    }
}

fn parse_encoding(input: &str) -> Result<Encoding, execute::ExecuteError> {
    input
        .parse()
        .map_err(|e: String| execute::ExecuteError::InvalidEncoding(e))
}

fn parse_kv_param<K, V>(input: &str) -> Result<(K, V), Box<dyn Error + Send + Sync + 'static>>
where
    K: FromStr,
    K::Err: Error + Send + Sync + 'static,
    V: FromStr,
    V::Err: Error + Send + Sync + 'static,
{
    let split_pos = input
        .find('=')
        .ok_or_else(|| format!("invalid key=value parameter"))?;
    Ok((input[..split_pos].parse()?, input[split_pos + 1..].parse()?))
}
