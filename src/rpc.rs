use std::{
    collections::HashMap,
    net::UdpSocket,
    sync::{Arc, Mutex},
};

use crossbeam_channel::{Receiver, Sender};

use crate::{
    message::{Message, Response},
    protocol::Protocol,
};

pub struct RPC {
    socket: UdpSocket,
    pending: Arc<Mutex<HashMap<String, Sender<Option<Response>>>>>,
}

impl RPC {
    pub fn new(addr: String) -> Self {
        let socket = UdpSocket::bind(addr).unwrap();
        Self {
            socket,
            pending: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn send_request(&self, msg: Message) -> Receiver<Option<Response>> {
        todo!()
    }
}

impl Protocol for RPC {
    fn ping(&self, node_addr: String) -> anyhow::Result<()> {
        todo!()
    }

    fn store(key: String, val: String) -> anyhow::Result<()> {
        todo!()
    }

    fn find_node(id: crate::guid::GUID) -> Vec<crate::node::Location> {
        todo!()
    }

    fn find_value(key: String) -> Option<String> {
        todo!()
    }

    fn get_value(key: String) -> Option<String> {
        todo!()
    }
}
