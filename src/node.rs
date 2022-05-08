use serde::{Deserialize, Serialize};

use crate::{guid::GUID_LEN, rpc::RPC};
use std::collections::HashMap;

use super::guid::GUID;

const BUCKET_LEN: usize = GUID_LEN * 8;

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub id: GUID,
    pub addr: String,
    pub port: String,
}

struct RoutingTable(Vec<Vec<Location>>);

impl RoutingTable {
    pub fn new() -> Self {
        let mut buckets = Vec::new();

        for i in 0..BUCKET_LEN {
            buckets.push(Vec::new())
        }

        Self(buckets)
    }
}
pub struct Node {
    location: Location,
    routing_table: RoutingTable,
    transport: RPC,
    store: HashMap<String, String>,
}

impl Node {
    pub fn new(location: Location) -> Self {
        let addr = location.addr.clone();
        Self {
            location,
            routing_table: RoutingTable::new(),
            transport: RPC::new(addr),
            store: HashMap::new(),
        }
    }
}
