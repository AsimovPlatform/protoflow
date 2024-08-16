// This is free and unencumbered software released into the public domain.

mod sysexits;

use std::path::PathBuf;

use crate::sysexits::Sysexits;
use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use protoflow_syntax::{Code, SystemParser};

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
    Config {},

    /// Check the syntax of a Protoflow system
    Check {
        /// Pathnames of Protoflow files to check
        #[clap(default_value = "/dev/stdin")]
        paths: Vec<PathBuf>,
    },

    #[cfg(feature = "dev")]
    /// Execute a Protoflow system or block
    Execute { path_or_uri: PathBuf },

    /// Generate code from a Protoflow system
    Generate {
        /// Pathname of the Protoflow file
        path: PathBuf,
    },
}

pub fn main() -> Sysexits {
    // Load environment variables from `.env`:
    dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = wild::args_os();
    let args = argfile::expand_args_from(args, argfile::parse_fromfile, argfile::PREFIX).unwrap();

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
        Commands::Config {} => Ok(()),
        Commands::Check { paths } => check(paths),
        #[cfg(feature = "dev")]
        Commands::Execute { path_or_uri } => execute(path_or_uri),
        Commands::Generate { path } => generate(path),
    };
    return result.err().unwrap_or_default();
}

fn version(_options: &Options) -> Result<(), Sysexits> {
    // TODO
    Ok(())
}

fn license() -> Result<(), Sysexits> {
    println!("This is free and unencumbered software released into the public domain.");
    Ok(())
}

fn check(paths: &Vec<PathBuf>) -> Result<(), Sysexits> {
    for path in paths {
        let mut parser = SystemParser::from_file(path)?;
        let _ = parser.check()?;
    }
    Ok(())
}

#[cfg(feature = "dev")]
fn execute(path_or_uri: &PathBuf) -> Result<(), Sysexits> {
    let path_or_uri = path_or_uri.to_string_lossy();
    match path_or_uri.as_ref() {
        _ => {} // TODO
    }
    Ok(())
}

fn generate(path: &PathBuf) -> Result<(), Sysexits> {
    let mut parser = SystemParser::from_file(path)?;
    let model = parser.check()?;
    let code = Code::try_from(model)?;
    std::print!("{}", code.unparse());
    Ok(())
}
