// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{
    prelude::{Bytes, String, ToString, Vec},
    StdioConfig, StdioError, StdioSystem, System,
};
use protoflow_core::{
    types::{Value, value::Kind::*}, Block, BlockError, BlockResult, BlockRuntime, InputPort, OutputPort,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;
use csv::WriterBuilder;
use std::io::Cursor;
/// A block that encodes CSV files by converting a header and rows, provided as `prost_types::Value` streams, into a byte stream.
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
///     // TODO
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute EncodeCsv
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
        runtime.wait_for(&self.rows)?;

        let mut buffer = Cursor::new(Vec::new());
        {
            let mut wtr = WriterBuilder::new()
                .has_headers(false)
                .from_writer(&mut buffer);

            if let Some(header) = self.header.recv()? {
                if let Some(ListValue(lv)) = header.kind {
                    let header_row: Vec<String> = lv.values
                        .into_iter()
                        .filter_map(|v| if let Some(StringValue(s)) = v.kind { Some(s) } else { None })
                        .collect();

                    wtr.write_record(header_row)
                        .map_err(|e| BlockError::Other(e.to_string()))?;
                }
            }

            while let Some(row) = self.rows.recv()? {
                if let Some(ListValue(lv)) = row.kind {
                    let row_values: Vec<String> = lv.values
                        .into_iter()
                        .filter_map(|v| if let Some(StringValue(s)) = v.kind { Some(s) } else { None })
                        .collect();

                    wtr.write_record(row_values)
                        .map_err(|e| BlockError::Other(e.to_string()))?;
                }
            }

            wtr.flush().map_err(|e| BlockError::Other(e.to_string()))?;
        }

        self.output.send(&Bytes::from(buffer.into_inner()))?;

        Ok(())
    }
}


#[cfg(feature = "std")]
impl StdioSystem for EncodeCsv {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {

        config.reject_any()?;

        Ok(System::build(|_s| { todo!() }))
    }
}

#[cfg(test)]
mod tests {

    use super::EncodeCsv;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(EncodeCsv::new(s.input(), s.input(), s.output()));
        });
    }
}