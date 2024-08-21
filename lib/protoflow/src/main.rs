// This is free and unencumbered software released into the public domain.

mod commands {
    #[cfg(feature = "beta")]
    pub mod check;
    #[cfg(feature = "beta")]
    pub mod config;
    pub mod execute;
    #[cfg(feature = "beta")]
    pub mod generate;
}
mod sysexits;

use crate::sysexits::Sysexits;
use clap::{Parser, Subcommand};
use clientele::dotenv;
use protoflow_blocks::Encoding;
use std::{error::Error, path::PathBuf, str::FromStr};

/// Protoflow command-line interface (CLI)
#[derive(Debug, Parser)]
#[command(name = "Protoflow", about)]
#[command(arg_required_else_help = true)]
struct Options {
    /// Enable debugging output
    #[clap(short = 'd', long, value_parser, global = true)]
    debug: bool,

    /// Show license information
    #[clap(long, value_parser)]
    license: bool,

    /// Enable verbose output
    #[clap(short = 'v', long, value_parser, global = true)]
    verbose: bool,

    /// Print version information
    #[clap(short = 'V', long, value_parser)]
    version: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
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

pub fn main() -> Sysexits {
    // Load environment variables from `.env`:
    dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os();
    let args =
        clientele::expand_args_from(args, clientele::parse_fromfile, clientele::PREFIX).unwrap();

    // Parse command-line options:
    let options = Options::parse_from(args);

    if options.version {
        return version(&options).err().unwrap_or_default();
    }

    if options.license {
        return license().err().unwrap_or_default();
    }

    if options.verbose || options.debug {
        // TODO: configure tracing
    }

    let subcommand = &options.command;
    let result = match subcommand.as_ref().expect("subcommand is required") {
        #[cfg(feature = "beta")]
        Commands::Config {} => Ok(()),
        #[cfg(feature = "beta")]
        Commands::Check { paths } => commands::check::check(paths),
        Commands::Execute {
            block,
            encoding,
            params,
        } => commands::execute::execute(block, params, *encoding),
        #[cfg(feature = "beta")]
        Commands::Generate { path } => commands::generate::generate(path),
    };
    return result.err().unwrap_or_default();
}

fn version(_options: &Options) -> Result<(), Sysexits> {
    println!("protoflow {}", env!("CARGO_PKG_VERSION"));
    Ok(())
}

fn license() -> Result<(), Sysexits> {
    println!("This is free and unencumbered software released into the public domain.");
    Ok(())
}

fn parse_encoding(input: &str) -> Result<Encoding, commands::execute::ExecuteError> {
    input
        .parse()
        .map_err(|e: String| commands::execute::ExecuteError::InvalidEncoding(e))
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
