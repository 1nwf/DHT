use std::time::Instant;

use crate::{guid::GUID, node::Location};
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum MessageType {
    Terminate,
    Request(Request),
    Response(Response),
}
#[derive(Serialize, Deserialize)]
pub enum Request {
    Ping,
    FindNode(GUID),
    Store(String, String),
    GetValue(String),
}
#[derive(Serialize, Deserialize)]
pub enum Response {
    Pong,
    FindNode(Location),
    GetValue(Option<String>),
}

pub struct Message {
    pub id: String,
    pub dist: Location,
    pub source: Location,
    pub msg: MessageType,
}

impl Message {
    pub fn new(msg: MessageType, source: Location, dist: Location) -> Self {
        Self {
            id: GUID::new(format!(
                "{}:{}:{:?}",
                source.id.to_hex(),
                dist.id.to_hex(),
                Local::now().timestamp_millis()
            ))
            .to_hex(),
            dist,
            source,
            msg,
        }
    }
}
