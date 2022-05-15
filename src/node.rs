use crate::{
    cast,
    guid::GUID_LEN,
    message::{FindValue, Message, MessageType, Request, Response},
    protocol::Protocol,
    routing::RoutingTable,
    rpc::{Rpc, CONCCURENT_REQS},
};
use anyhow::Ok;
use crossbeam_channel::Receiver;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Display,
    sync::{Arc, Mutex},
    thread,
};

use super::guid::GUID;

pub const BUCKET_LEN: usize = GUID_LEN * 8;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Location {
    pub id: GUID,
    pub ip: String,
    pub port: u16,
}

impl Location {
    pub fn new(ip: String, port: u16) -> Self {
        let id = GUID::new(format!("{}:{}", ip, port.clone()));
        Self { id, ip, port }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.addr())
    }
}

impl From<&str> for Location {
    fn from(addr: &str) -> Self {
        let items: Vec<_> = addr.split(':').collect();
        if items.len() != 2 {
            panic!("cannot convert string to location");
        } else {
            Self {
                id: GUID::new(addr.to_string()),
                ip: items[0].to_string(),
                port: items[1].parse().unwrap(),
            }
        }
    }
}

#[derive(Clone)]
pub struct Node {
    pub routing_table: Arc<Mutex<RoutingTable>>,
    pub transport: Arc<Rpc>,
    db: Arc<Mutex<HashMap<String, String>>>,
    _receiver: Receiver<Message>,
}

impl Node {
    pub fn print_routing(&self) {
        self.routing_table.lock().unwrap().print_buckets()
    }
    pub fn new(location: Location, bootstrap: Option<Location>) -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();

        let rpc = Rpc::new(location);
        Rpc::listen(rpc.clone(), sender);

        let routing_table = Arc::new(Mutex::new(RoutingTable::new()));

        let node = Self {
            routing_table,
            transport: Arc::new(rpc),
            db: Arc::new(Mutex::new(HashMap::new())),
            _receiver: receiver,
        };

        if let Some(bts) = bootstrap {
            node.routing_table.lock().unwrap().insert(node.id(), bts);
        }

        node
    }

    pub fn open(&self) {
        let mut node = self.clone();

        thread::spawn(move || loop {
            let msg = node._receiver.recv().unwrap();
            node.handle_req(msg);
        });
    }

    pub fn handle_req(&mut self, req: Message) {
        let req_type = cast!(req.msg, MessageType::Request);
        let res: MessageType = match req_type {
            Request::Ping => self.handle_ping(),
            Request::FindNode(id) => self.handle_find_node(id),
            Request::Store(key, val) => self.handle_store(key, val),
            Request::FindValue(key) => self.handle_find_value(key),
            Request::Join => self.handle_join(req.source.clone()),
        };

        let msg = Message::new_res(req.id, res, req.dist, req.source);
        self.transport.send_msg(msg).unwrap();
    }

    pub fn handle_ping(&self) -> MessageType {
        MessageType::Response(Response::Pong)
    }

    pub fn handle_store(&mut self, key: String, val: String) -> MessageType {
        Arc::clone(&self.db).lock().unwrap().insert(key, val);
        MessageType::Response(Response::Store)
    }

    pub fn handle_find_value(&self, key: String) -> MessageType {
        if let Some(val) = self.db.lock().unwrap().get(&key) {
            MessageType::Response(Response::FindValue(FindValue::Value(val.clone())))
        } else {
            let nodes = cast!(
                cast!(self.handle_find_node(GUID::new(key)), MessageType::Response),
                Response::FindNode
            );
            MessageType::Response(Response::FindValue(FindValue::ClosestNodes(nodes)))
        }
    }

    pub fn print_store(&self) {
        for (key, val) in self.db.lock().unwrap().iter() {
            println!("{key}: {val}");
        }
    }

    pub fn location(&self) -> Location {
        self.transport.location.clone()
    }

    pub fn handle_find_node(&self, key: GUID) -> MessageType {
        todo!()
    }

    pub fn handle_join(&mut self, dist: Location) -> MessageType {
        self.routing_table.lock().unwrap().insert(self.id(), dist);
        MessageType::Response(Response::Join)
    }

    pub fn id(&self) -> GUID {
        self.transport.location.id
    }
}

impl Protocol for Node {
    fn ping(&self, dist: Location) -> anyhow::Result<()> {
        let msg = Message::new_req(
            MessageType::Request(Request::Ping),
            self.location(),
            dist.clone(),
        );
        match self.transport.send_request(msg).recv().unwrap() {
            Some(_) => {
                self.routing_table
                    .lock()
                    .unwrap()
                    .insert(self.transport.location.id, dist);
                Ok(())
            }
            None => {
                self.routing_table
                    .lock()
                    .unwrap()
                    .remove(&self.transport.location.id, &dist.id);
                Err(anyhow::anyhow!("dead node"))
            }
        }
    }

    fn store(
        &mut self,
        key: String,
        val: String,
        dist: crate::node::Location,
    ) -> anyhow::Result<()> {
        let msg = Message::new_req(
            MessageType::Request(Request::Store(key, val)),
            self.location(),
            dist,
        );
        match self
            .transport
            .send_request(msg)
            .recv()
            .expect("unable to send store request")
        {
            Some(_) => Ok(()),
            None => Err(anyhow::anyhow!("dead node")),
        }
    }

    fn find_node(&self, id: GUID, dist: Location) -> Option<Vec<Location>> {
        let msg = Message::new_req(
            MessageType::Request(Request::FindNode(id)),
            self.location(),
            dist.clone(),
        );
        match self.transport.send_request(msg).recv().unwrap() {
            Some(res) => {
                let val = cast!(res, Response::FindNode);
                self.routing_table
                    .lock()
                    .unwrap()
                    .insert(self.location().id, dist);
                Some(val)
            }
            None => {
                self.routing_table
                    .lock()
                    .unwrap()
                    .remove(&self.transport.location.id, &dist.id);
                None
            }
        }
    }

    fn find_value(&self, key: String, dist: Location) -> FindValue {
        let msg = Message::new_req(
            MessageType::Request(Request::FindValue(key)),
            self.location(),
            dist,
        );
        let res = self.transport.send_request(msg).recv().unwrap().unwrap();
        let val = cast!(res, Response::FindValue);
        val
    }

    fn join(&mut self, dist: Location) -> bool {
        let msg = Message::new_req(
            MessageType::Request(Request::Join),
            self.location(),
            dist.clone(),
        );

        let res = self.transport.send_request(msg).recv().unwrap();
        match res {
            Some(_) => {
                self.routing_table.lock().unwrap().insert(self.id(), dist);
                true
            }
            None => false,
        }
    }
}
