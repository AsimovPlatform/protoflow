// This is free and unencumbered software released into the public domain.

use crate::sysexits::Sysexits;
use protoflow_syntax::{Code, SystemParser};
use std::path::PathBuf;

#[derive(Debug)]
pub enum GenerateError {}

pub fn generate(path: &PathBuf) -> Result<(), Sysexits> {
    let mut parser = SystemParser::from_file(path)?;
    let model = parser.check()?;
    let code = Code::try_from(model)?;
    std::print!("{}", code.unparse());
    Ok(())
}
