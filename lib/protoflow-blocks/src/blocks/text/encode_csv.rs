// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{
    prelude::{Bytes, ToString, vec},
    StdioConfig, StdioError, StdioSystem, System,
};
use protoflow_core::{
    types::{Value, value::Kind::*, ListValue as LV}, Block, BlockError, BlockResult, BlockRuntime, InputPort, OutputPort,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;
use csv::ReaderBuilder;
use std::io::Cursor;
/// A block that encodes csv files.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/text/encode_csv.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/text/encode_csv.seq.mmd" framed)]
///
/// # Examples
///
/// ## Using the block in a system
///
/// ```rust
/// # use protoflow_blocks::*;
/// # fn main() {
/// System::build(|s| {
///     todo
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute DecodeCsv path="file.csv"
/// ```
///