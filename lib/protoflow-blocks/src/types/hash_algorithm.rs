// This is free and unencumbered software released into the public domain.

use crate::prelude::{fmt, Box, FromStr, String, Vec};

/// The cryptographic hash algorithm to use.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HashAlgorithm {
    #[cfg(feature = "hash-blake3")]
    BLAKE3,
    #[cfg(feature = "hash-sha2")]
    SHA256,
    #[cfg(feature = "hash-sha1")]
    SHA1,
    #[cfg(feature = "hash-md5")]
    MD5,
}

impl FromStr for HashAlgorithm {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use HashAlgorithm::*;
        Ok(match input.to_lowercase().as_str() {
            #[cfg(feature = "hash-blake3")]
            "blake3" | "b3" => BLAKE3,
            #[cfg(feature = "hash-sha2")]
            "sha256" => SHA256,
            #[cfg(feature = "hash-sha1")]
            "sha1" => SHA1,
            #[cfg(feature = "hash-md5")]
            "md5" => MD5,
            _ => return Err(String::from(input)),
        })
    }
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use HashAlgorithm::*;
        match self {
            #[cfg(feature = "hash-blake3")]
            BLAKE3 => write!(f, "blake3"),
            #[cfg(feature = "hash-sha2")]
            SHA256 => write!(f, "sha256"),
            #[cfg(feature = "hash-sha1")]
            SHA1 => write!(f, "sha1"),
            #[cfg(feature = "hash-md5")]
            MD5 => write!(f, "md5"),
        }
    }
}

impl Default for HashAlgorithm {
    fn default() -> Self {
        #[cfg(feature = "hash-blake3")]
        {
            return HashAlgorithm::BLAKE3;
        }
        #[cfg(all(not(feature = "hash-blake3"), feature = "hash-sha2"))]
        {
            return HashAlgorithm::SHA256;
        }
        #[cfg(all(
            not(feature = "hash-blake3"),
            not(feature = "hash-sha2"),
            feature = "hash-sha1"
        ))]
        {
            return HashAlgorithm::SHA1;
        }
        #[cfg(all(
            not(feature = "hash-blake3"),
            not(feature = "hash-sha2"),
            not(feature = "hash-sha1"),
            feature = "hash-md5"
        ))]
        {
            return HashAlgorithm::MD5;
        }
    }
}

pub trait Hasher {
    fn compute_hash(&self, data: &[u8]) -> Vec<u8>;
}

#[cfg(feature = "hash-blake3")]
struct Blake3 {}

#[cfg(feature = "hash-blake3")]
impl Hasher for Blake3 {
    fn compute_hash(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(data);
        hasher.finalize().as_bytes().to_vec()
    }
}

#[cfg(feature = "hash-sha2")]
struct Sha256 {}

#[cfg(feature = "hash-sha2")]
impl Hasher for Sha256 {
    fn compute_hash(&self, data: &[u8]) -> Vec<u8> {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

#[cfg(feature = "hash-sha1")]
struct Sha1 {}

#[cfg(feature = "hash-sha1")]
impl Hasher for Sha1 {
    fn compute_hash(&self, data: &[u8]) -> Vec<u8> {
        use sha1::{Digest, Sha1};
        let mut hasher = Sha1::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

#[cfg(feature = "hash-md5")]
struct Md5 {}

#[cfg(feature = "hash-md5")]
impl Hasher for Md5 {
    fn compute_hash(&self, data: &[u8]) -> Vec<u8> {
        use md5::{Digest, Md5};
        let mut hasher = Md5::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

pub struct HasherFactory;

impl HasherFactory {
    pub fn new(algorithm: HashAlgorithm) -> Box<dyn Hasher> {
        use HashAlgorithm::*;
        match algorithm {
            #[cfg(feature = "hash-blake3")]
            BLAKE3 => Box::new(Blake3 {}),
            #[cfg(feature = "hash-md5")]
            MD5 => Box::new(Md5 {}),
            #[cfg(feature = "hash-sha1")]
            SHA1 => Box::new(Sha1 {}),
            #[cfg(feature = "hash-sha2")]
            SHA256 => Box::new(Sha256 {}),
        }
    }
}
