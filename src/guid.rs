use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::io::Write;

pub const GUID_LEN: usize = 32; // 256 bytes
#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub fn to_hex(&self) -> String {
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

pub trait Distance {
    fn distance_from(&self, b: &Self) -> [u8; GUID_LEN];
}

impl Distance for GUID {
    fn distance_from(&self, b: &Self) -> [u8; GUID_LEN] {
        let mut res = [0; GUID_LEN];
        for (i, val) in self.0.iter().enumerate() {
            res[i] = val ^ b.0[i]
        }

        res
    }
}

impl Distance for [u8; GUID_LEN] {
    fn distance_from(&self, b: &Self) -> [u8; GUID_LEN] {
        let mut res = [0; GUID_LEN];
        for (i, val) in self.iter().enumerate() {
            res[i] = val ^ b[i]
        }
        res
    }
}
