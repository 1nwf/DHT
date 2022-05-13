use crate::{guid::GUID, node::Location};
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    Terminate,
    Request(Request),
    Response(Response),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    Ping,
    FindNode(GUID),
    Store(String, String),
    FindValue(String),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Response {
    Pong,
    FindNode(Vec<Location>),
    Store,
    FindValue(FindValue),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FindValue {
    Value(String),
    ClosestNodes(Vec<Location>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: String,
    pub dist: Location,
    pub source: Location,
    pub msg: MessageType,
}

impl Message {
    pub fn new_req(msg: MessageType, source: Location, dist: Location) -> Self {
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

    pub fn new_res(req_id: String, msg: MessageType, source: Location, dist: Location) -> Self {
        Self {
            id: req_id,
            dist,
            source,
            msg,
        }
    }
}
