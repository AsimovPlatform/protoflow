// This is free and unencumbered software released into the public domain.

pub trait HashBlocks {
    fn hash_blake3(&self) -> Hash;
}

mod hash;
pub use hash::*;
