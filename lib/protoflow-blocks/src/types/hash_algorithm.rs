// This is free and unencumbered software released into the public domain.

use crate::prelude::{fmt, Box, FromStr, String, Vec};

/// The cryptographic hash algorithm to use.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HashAlgorithm {
    #[default]
    BLAKE3,
    #[cfg(feature = "sha2")]
    SHA256,
    #[cfg(feature = "sha1")]
    SHA1,
    #[cfg(feature = "md-5")]
    MD5,
}

impl FromStr for HashAlgorithm {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use HashAlgorithm::*;
        Ok(match input.to_lowercase().as_str() {
            "blake3" | "b3" => BLAKE3,
            #[cfg(feature = "sha2")]
            "s256" | "sha256" => SHA256,
            #[cfg(feature = "sha1")]
            "s1" | "sha1" => SHA1,
            #[cfg(feature = "md-5")]
            "m5" | "md5" => MD5,
            _ => return Err(String::from(input)),
        })
    }
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use HashAlgorithm::*;
        match self {
            BLAKE3 => write!(f, "blake3"),
            #[cfg(feature = "sha2")]
            SHA256 => write!(f, "sha256"),
            #[cfg(feature = "sha1")]
            SHA1 => write!(f, "sha1"),
            #[cfg(feature = "md-5")]
            MD5 => write!(f, "md5"),
        }
    }
}

pub trait Hasher {
    fn compute_hash(&self, data: &[u8]) -> Vec<u8>;
}

#[cfg(feature = "hash")]
struct Blake3 {}

#[cfg(feature = "hash")]
impl Hasher for Blake3 {
    fn compute_hash(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(data);
        hasher.finalize().as_bytes().to_vec()
    }
}

#[cfg(feature = "sha2")]
struct Sha256 {}

#[cfg(feature = "sha2")]
impl Hasher for Sha256 {
    fn compute_hash(&self, data: &[u8]) -> Vec<u8> {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

#[cfg(feature = "sha1")]
struct Sha1 {}

#[cfg(feature = "sha1")]
impl Hasher for Sha1 {
    fn compute_hash(&self, data: &[u8]) -> Vec<u8> {
        use sha1::{Digest, Sha1};
        let mut hasher = Sha1::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

#[cfg(feature = "md-5")]
struct Md5 {}

#[cfg(feature = "md-5")]
impl Hasher for Md5 {
    fn compute_hash(&self, data: &[u8]) -> Vec<u8> {
        use md5::{Digest, Md5};
        let mut hasher = Md5::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

#[cfg(feature = "hash")]
pub struct HasherFactory;

#[cfg(feature = "hash")]
impl HasherFactory {
    pub fn new(algorithm: HashAlgorithm) -> Box<dyn Hasher> {
        use HashAlgorithm::*;
        match algorithm {
            BLAKE3 => Box::new(Blake3 {}),
            #[cfg(feature = "sha2")]
            SHA256 => Box::new(Sha256 {}),
            #[cfg(feature = "sha1")]
            SHA1 => Box::new(Sha1 {}),
            #[cfg(feature = "md-5")]
            MD5 => Box::new(Md5 {}),
        }
    }
}
