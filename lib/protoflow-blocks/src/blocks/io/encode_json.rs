// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{
    prelude::{Bytes, Vec},
    StdioConfig, StdioError, StdioSystem, System,
};
use protoflow_core::{types::Value, Block, BlockResult, BlockRuntime, InputPort, OutputPort};
use protoflow_derive::Block;
use simple_mermaid::mermaid;
use struson::writer::*;

/// A block that encodes messages into JSON format.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/io/encode_hex.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/io/encode_hex.seq.mmd" framed)]
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
/// $ protoflow execute EncodeJSON
/// ```
///
#[derive(Block, Clone)]
pub struct EncodeJson {
    /// The input message stream.
    #[input]
    pub input: InputPort<Value>,

    /// The output byte stream.
    #[output]
    pub output: OutputPort<Bytes>,
}

impl EncodeJson {
    pub fn new(input: InputPort<Value>, output: OutputPort<Bytes>) -> Self {
        Self { input, output }
    }

    pub fn with_system(system: &System) -> Self {
        use crate::SystemBuilding;
        Self::new(system.input(), system.output())
    }
}

impl Block for EncodeJson {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        runtime.wait_for(&self.input)?;

        while let Some(input) = self.input.recv()? {
            let mut buffer = Vec::<u8>::new();
            {
                let mut json = JsonStreamWriter::new(&mut buffer);
                encode(&mut json, input)?;
                json.finish_document()?;
            }
            let output = Bytes::from(buffer);
            self.output.send(&output)?;
        }

        Ok(())
    }
}

fn encode(json: &mut JsonStreamWriter<&mut Vec<u8>>, value: Value) -> Result<(), std::io::Error> {
    use protoflow_core::types::value::Kind::*;
    match value.kind.unwrap() {
        NullValue(_) => json.null_value()?,
        BoolValue(bool) => json.bool_value(bool)?,
        NumberValue(number) => {
            if number.is_nan() {
                json.string_value("NaN")?
            } else if number.is_infinite() && number.is_sign_negative() {
                json.string_value("-Infinity")?
            } else if number.is_infinite() && number.is_sign_positive() {
                json.string_value("Infinity")?
            } else {
                json.fp_number_value(number).map_err(|_| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "failed to encode JSON number",
                    )
                })?;
            }
        }
        StringValue(string) => json.string_value(&string)?,
        ListValue(array) => {
            json.begin_array()?;
            for value in array.values {
                encode(json, value)?;
            }
            json.end_array()?
        }
        StructValue(object) => {
            json.begin_object()?;
            for (key, value) in object.fields {
                json.name(&key)?;
                encode(json, value)?;
            }
            json.end_object()?
        }
    }
    Ok(())
}

#[cfg(feature = "std")]
impl StdioSystem for EncodeJson {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        //use crate::SystemBuilding;

        config.reject_any()?;

        Ok(System::build(|_s| {
            //let stdin = config.read_stdin(s);
            //let json_encoder = s.encode_json();
            //let stdout = config.write_stdout(s);
            //s.connect(&stdin.output, &json_encoder.input);
            //s.connect(&json_encoder.output, &stdout.input);
            todo!() // TODO
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::EncodeJson;
    use crate::{System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(EncodeJson::new(s.input(), s.output()));
        });
    }
}
