// This is free and unencumbered software released into the public domain.

use crate::prelude::{Bytes, String, Vec};

pub trait Message: prost::Message + Clone + Default {}

impl Message for bool {} // google.protobuf.BoolValue
impl Message for u32 {} // google.protobuf.UInt32Value
impl Message for u64 {} // google.protobuf.UInt64Value
impl Message for i32 {} // google.protobuf.Int32Value
impl Message for i64 {} // google.protobuf.Int64Value
impl Message for f32 {} // google.protobuf.FloatValue
impl Message for f64 {} // google.protobuf.DoubleValue
impl Message for String {} // google.protobuf.StringValue
impl Message for Vec<u8> {} // google.protobuf.BytesValue
impl Message for Bytes {} // google.protobuf.BytesValue
impl Message for () {} // google.protobuf.Empty

impl Message for prost_types::Any {} // google.protobuf.Any
impl Message for prost_types::Duration {} // google.protobuf.Duration
impl Message for prost_types::ListValue {} // google.protobuf.ListValue
impl Message for prost_types::Option {} // google.protobuf.Option
impl Message for prost_types::Struct {} // google.protobuf.Struct
impl Message for prost_types::Timestamp {} // google.protobuf.Timestamp
impl Message for prost_types::Value {} // google.protobuf.Value
