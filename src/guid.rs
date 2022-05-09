use std::io::Write;

use serde::{Deserialize, Serialize};
use sha2::Digest;

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
}

#[derive(Debug)]
pub struct Distance(pub [u8; GUID_LEN]);

impl Distance {
    pub fn calc(a: &GUID, b: &GUID) -> Self {
        let mut res = [0; GUID_LEN];
        for (i, val) in a.0.iter().enumerate() {
            res[i] = val ^ b.0[i]
        }
        Distance(res)
    }

    pub fn get_bucket_idx() -> usize {
        todo!()
    }
}
