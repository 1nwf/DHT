use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::io::Write;

pub const GUID_LEN: usize = 32; // 256 bytes
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Copy)]
pub struct GUID(pub [u8; GUID_LEN]);

impl GUID {
    pub fn new(data: String) -> Self {
        let mut hasher = sha2::Sha256::new();
        let _ = hasher.write(data.as_bytes()).expect("unable to hash data");
        let res = hasher.finalize();
        let mut hash = [0; GUID_LEN];

        for i in 0..res.len() {
            hash[i] = res[i]
        }

        Self(hash)
    }
    pub fn to_hex(self) -> String {
        hex::encode(self.0)
    }

    pub fn from_hex(hex_str: String) -> Self {
        let mut data = [0u8; GUID_LEN];
        let decoded_hex = hex::decode(hex_str).unwrap();
        for (idx, val) in decoded_hex.iter().enumerate() {
            data[idx] = *val;
        }
        Self(data)
    }
}

impl From<&str> for GUID {
    fn from(data: &str) -> Self {
        Self::new(data.to_string())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Distance(pub [u8; GUID_LEN]);
impl Distance {
    pub fn calc(id1: &GUID, id2: &GUID) -> Self {
        let mut dist = [0; GUID_LEN];
        for ((idx, v1), v2) in id1.0.iter().enumerate().zip(id2.0) {
            dist[idx] = v1 ^ v2
        }
        Self(dist)
    }
}
