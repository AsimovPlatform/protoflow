// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{
    prelude::{BTreeMap, Bytes, ToString, Vec},
    StdioConfig, StdioError, StdioSystem, System,
};
use protoflow_core::{
    types::Value, Block, BlockError, BlockResult, BlockRuntime, InputPort, OutputPort,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;
use struson::reader::*;

/// A block that decodes a JSON format message from a byte stream.
///
/// # Block Diagram
#[doc = mermaid!("../../../doc/io/decode_json.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../../doc/io/decode_json.seq.mmd" framed)]
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
/// $ protoflow execute DecodeJSON
/// ```
///
#[derive(Block, Clone)]
pub struct DecodeJson {
    /// The input message byte stream.
    #[input]
    pub input: InputPort<Bytes>,

    /// The output JSON value.
    #[output]
    pub output: OutputPort<Value>,
}

impl DecodeJson {
    pub fn new(input: InputPort<Bytes>, output: OutputPort<Value>) -> Self {
        Self { input, output }
    }

    pub fn with_system(system: &System) -> Self {
        use crate::SystemBuilding;
        Self::new(system.input(), system.output())
    }
}

impl Block for DecodeJson {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        runtime.wait_for(&self.input)?;

        while let Some(input) = self.input.recv()? {
            let output = {
                let mut json = JsonStreamReader::new(input.as_ref());
                decode(&mut json).map_err(|e| BlockError::Other(e.to_string()))?
            };
            self.output.send(&output)?;
        }

        Ok(())
    }
}

pub fn decode(json: &mut JsonStreamReader<&[u8]>) -> Result<Value, ReaderError> {
    use protoflow_core::types::{value::Kind::*, ListValue as LV, Struct};

    let kind = match json.peek()? {
        ValueType::Null => json.next_null().map(|_| NullValue(0))?,
        ValueType::Boolean => json.next_bool().map(BoolValue)?,
        ValueType::Number => json.next_number_as_string().and_then(|num_str| {
            Ok(match num_str.as_str() {
                "NaN" => NumberValue(f64::NAN),
                "-Infinity" => NumberValue(f64::NEG_INFINITY),
                "Infinity" => NumberValue(f64::INFINITY),
                number => NumberValue(number.parse::<f64>().map_err(|_| {
                    ReaderError::UnsupportedNumberValue {
                        number: number.into(),
                        location: json.current_position(false),
                    }
                })?),
            })
        })?,
        ValueType::String => json.next_string().map(StringValue)?,
        ValueType::Array => {
            json.begin_array()?;

            let mut values = Vec::new();
            while json.has_next().unwrap() {
                values.push(decode(json)?);
            }

            json.end_array()?;
            ListValue(LV { values })
        }
        ValueType::Object => {
            json.begin_object()?;
            let mut fields = BTreeMap::new();
            while json.has_next()? {
                let key = json.next_name_owned()?;
                let value = decode(json)?;
                fields.insert(key, value);
            }
            json.end_object()?;
            StructValue(Struct { fields })
        }
    };

    Ok(Value { kind: Some(kind) })
}

#[cfg(feature = "std")]
impl StdioSystem for DecodeJson {
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        //use crate::{IoBlocks, SysBlocks, SystemBuilding};

        config.reject_any()?;

        Ok(System::build(|_s| {
            //let stdin = s.read_stdin();
            //let json_decoder = s.decode_json();
            //let stdout = s.write_stdout();
            //s.connect(&stdin.output, &json_decoder.input);
            //s.connect(&json_decoder.output, &stdout.input);
            todo!() // TODO
        }))
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::{decode, DecodeJson};
    use crate::{
        prelude::{vec, BTreeMap, ToString},
        System, SystemBuilding,
    };

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(DecodeJson::new(s.input(), s.output()));
        });
    }

    #[test]
    fn test_decode() {
        use super::{JsonStreamReader, ReaderError, Value};

        use protoflow_core::types::{
            value::Kind::{self, *},
            ListValue as LV, Struct,
        };

        fn to_value(kind: Kind) -> Value {
            Value { kind: Some(kind) }
        }

        fn read_value(input: &str) -> Result<Value, ReaderError> {
            let mut json = JsonStreamReader::new(input.as_ref());
            decode(&mut json)
        }

        assert_eq!(to_value(NullValue(0)), read_value("null").unwrap());
        assert_eq!(to_value(BoolValue(false)), read_value("false").unwrap());
        assert_eq!(to_value(BoolValue(true)), read_value("true").unwrap());
        assert_eq!(to_value(NumberValue(0.)), read_value("0").unwrap());
        assert_eq!(to_value(NumberValue(1.)), read_value("1").unwrap());
        assert_eq!(to_value(NumberValue(1.)), read_value("1").unwrap());
        assert_eq!(to_value(NumberValue(1.)), read_value("1.0").unwrap());
        assert_eq!(to_value(NumberValue(1.123)), read_value("1.123").unwrap());
        assert_eq!(to_value(NumberValue(-1.123)), read_value("-1.123").unwrap());
        assert_eq!(to_value(NumberValue(100.)), read_value("100").unwrap());
        assert_eq!(to_value(NumberValue(1000.)), read_value("1e3").unwrap());

        assert_eq!(
            to_value(StringValue("Hello world".to_string())),
            read_value(r#""Hello world""#).unwrap()
        );

        assert_eq!(
            to_value(ListValue(LV { values: vec![] })),
            read_value(r#"[]"#).unwrap()
        );

        assert_eq!(
            to_value(ListValue(LV {
                values: vec![
                    NullValue(0),
                    BoolValue(false),
                    BoolValue(true),
                    NumberValue(1.),
                    StringValue("foo".to_string()),
                    ListValue(LV {
                        values: vec![to_value(StringValue("nested".to_string()))]
                    }),
                    StructValue(Struct {
                        fields: {
                            let mut obj = BTreeMap::new();
                            obj.insert("foo".into(), to_value(NumberValue(1.)));
                            obj.insert("bar".into(), to_value(BoolValue(true)));
                            obj
                        }
                    }),
                ]
                .into_iter()
                .map(to_value)
                .collect()
            })),
            read_value(r#"[null, false, true, 1, "foo", ["nested"], {"foo": 1, "bar": true}]"#)
                .unwrap()
        );
    }
}
