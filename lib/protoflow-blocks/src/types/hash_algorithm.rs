// This is free and unencumbered software released into the public domain.

use crate::prelude::{fmt, FromStr, String, Vec};

/// The cryptographic hash algorithm to use.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HashAlgorithm {
    #[default]
    BLAKE3,
    SHA256,
    SHA1,
    MD5,
}

impl FromStr for HashAlgorithm {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use HashAlgorithm::*;
        Ok(match input.to_lowercase().as_str() {
            "blake3" | "b3" => BLAKE3,
            "s256" | "sha256" => SHA256,
            "s1" | "sha1" => SHA1,
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
            SHA256 => write!(f, "sha256"),
            SHA1 => write!(f, "sha1"),
            MD5 => write!(f, "md5"),
        }
    }
}
impl HashAlgorithm {
    pub fn compute_hash(&self, data: &[u8]) -> Vec<u8> {
        match self {
            HashAlgorithm::BLAKE3 => {
                let mut hasher = blake3::Hasher::new();
                hasher.update(data);
                hasher.finalize().as_bytes().to_vec()
            }
            HashAlgorithm::SHA256 => {
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::SHA1 => {
                use sha1::{Digest, Sha1};
                let mut hasher = Sha1::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::MD5 => {
                use md5::{Digest, Md5};
                let mut hasher = Md5::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
        }
    }
}
