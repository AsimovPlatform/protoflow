// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{
    prelude::{Bytes, ToString, vec},
    StdioConfig, StdioError, StdioSystem, System,
};
use protoflow_core::{
    types::Value, Block, BlockError, BlockResult, BlockRuntime, InputPort, OutputPort,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;
use csv::ReaderBuilder;
use std::io::{Bytes, Cursor};
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
#[derive(Block, Clone)]
pub struct EncodeCsv {
    /// The header message proto_types::Value stream.
    #[input]
    pub header: InputPort<Value>,
    /// The rows message proto_types::Value stream.
    #[input]
    pub rows: InputPort<Value>,
    /// The output message stream.
    #[output]
    pub output: OutputPort<Bytes>,
}

impl EncodeCsv {
    pub fn new(header: InputPort<Value>, rows: InputPort<Value>, output: OutputPort<Bytes>) -> Self {
        Self { header, rows, output }
    }

    pub fn with_system(system: &System) -> Self {
        use crate::SystemBuilding;
        Self::new(system.input(), system.input(), system.output())
    }
}

impl Block for EncodeCsv {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        runtime.wait_for(&self.header)?;

        while let (Some(header), Some(rows)) = (self.header.recv()?, self.rows.recv()?) {

        }

        Ok(())
    }
}