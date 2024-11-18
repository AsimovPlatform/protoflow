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
/// A block that decodes csv files.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/text/decode_csv.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/text/decode_csv.seq.mmd" framed)]
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
pub struct DecodeCsv {
    /// The input message bytes stream.
    #[input]
    pub input: InputPort<Bytes>,
    /// The csv header message.
    #[output]
    pub header: OutputPort<Value>,
    /// The csv rows message stream.
    #[output]
    pub rows: OutputPort<Value>,
}

impl DecodeCsv {
    pub fn new(input: InputPort<Bytes>, header: OutputPort<Value>, rows: OutputPort<Value>) -> Self {
        Self { input, header, rows }
    }

    pub fn with_system(system: &System) -> Self {
        use crate::SystemBuilding;
        Self::new(system.input(), system.output(), system.output())
    }
}

impl Block for DecodeCsv {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        runtime.wait_for(&self.input)?;

        while let Some(input) = self.input.recv()? {
            let cursor = Cursor::new(input);
            let mut rdr = ReaderBuilder::new()
                .has_headers(true)
                .from_reader(cursor);

            let headers = rdr.headers().map_err(|e| BlockError::Other(e.to_string()))?
                .iter()
                .map(|h| Value {
                    kind: Some(StringValue(h.to_string())),
                })
                .collect();

            let header_output = Value {
                kind: Some(ListValue(LV { values: headers }))
            };

            self.header.send(&header_output)?;

            for result in rdr.records() {
                let record = result.map_err(|e| BlockError::Other(e.to_string()))?;
                let row_values = record.iter()
                    .map(|v| Value {
                        kind: Some(StringValue(v.to_string())),
                    })
                    .collect();

                let row_output = Value {
                    kind: Some(ListValue(LV { values: row_values }))
                };

                self.rows.send(&row_output)?;
            }
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl StdioSystem for DecodeCsv {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        // use crate::{TextBlocks, IoBlocks, CoreBlocks, SysBlocks, SystemBuilding};

        config.allow_only(vec!["path"])?;
        // let path = config.get_string("path")?;

        Ok(System::build(|_s| {
            // let path = s.const_string(path);
            // let read_file = s.read_file();
            // let decode_csv = s.decode_csv();
            // let encoder = s.encode_with(config.encoding);
            // let stdout = config.write_stdout(s);
            // s.connect(&path.output, &read_file.path);
            // s.connect(&read_file.output, &decode_csv.input);
            // s.connect(&decode_csv.header, &encoder.input);
            // s.connect(&decode_csv.rows, &encoder.input);
            // s.connect(&encoder.output, &stdout.input);
            todo!()
        }))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(DecodeCsv::new(s.input(), s.output(), s.output()));
        });
    }
}
