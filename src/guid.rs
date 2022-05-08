use serde::{Deserialize, Serialize};

pub const GUID_LEN: usize = 32; // 256 bytes
#[derive(Debug, Serialize, Deserialize)]
pub struct GUID(pub [u8; GUID_LEN]);

impl GUID {
    pub fn new(data: String) -> Self {
        todo!()
    }
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

#[derive(Debug)]
pub struct Distance(pub [u8; GUID_LEN]);

impl Distance {
    pub fn calc(a: &GUID, b: &GUID) -> Self {
        todo!()
    }

    pub fn get_bucket_idx() -> usize {
        todo!()
    }
}
