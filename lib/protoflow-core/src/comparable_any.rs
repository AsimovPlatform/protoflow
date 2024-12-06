// This is free and unencumbered software released into the public domain.

use core::{
    cmp::Ordering,
    ops::{Deref, DerefMut},
};

use crate::{prelude::prost_types::Any, Message};

#[derive(Debug, Clone, Default)]
pub struct ComparableAny(pub Any);

impl PartialOrd for ComparableAny {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.type_url.cmp(&other.0.type_url))
    }
}

impl PartialEq for ComparableAny {
    fn eq(&self, other: &Self) -> bool {
        self.0.type_url == other.0.type_url
    }
}

impl From<Any> for ComparableAny {
    fn from(any: Any) -> Self {
        ComparableAny(any)
    }
}

impl From<ComparableAny> for Any {
    fn from(comparable: ComparableAny) -> Self {
        comparable.0
    }
}

impl AsRef<Any> for ComparableAny {
    fn as_ref(&self) -> &Any {
        &self.0
    }
}

impl AsMut<Any> for ComparableAny {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.0
    }
}

impl Deref for ComparableAny {
    type Target = Any;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ComparableAny {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl prost::Message for ComparableAny {
    fn encode_raw(&self, buf: &mut impl bytes::BufMut) {
        self.0.encode_raw(buf);
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: prost::encoding::WireType,
        buf: &mut impl bytes::Buf,
        ctx: prost::encoding::DecodeContext,
    ) -> Result<(), prost::DecodeError> {
        self.0.merge_field(tag, wire_type, buf, ctx)
    }

    fn encoded_len(&self) -> usize {
        self.0.encoded_len()
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}

impl Message for ComparableAny {}
