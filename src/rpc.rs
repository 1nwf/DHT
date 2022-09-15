use std::{
    collections::HashMap,
    net::UdpSocket,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crossbeam_channel::{Receiver, Sender};

use crate::{
    message::{Message, MessageType, Response},
    node::Location,
};

const TIMEOUT: Duration = Duration::new(5, 0);
pub const CONCCURENT_REQS: usize = 3;

#[derive(Debug, Clone)]
pub struct Rpc {
    socket: Arc<UdpSocket>,
    pub location: Location,
    pending: Arc<Mutex<HashMap<String, Sender<Option<Response>>>>>,
}

impl Rpc {
    pub fn new(location: Location) -> Self {
        let socket = UdpSocket::bind(location.addr()).unwrap();
        Self {
            location,
            socket: Arc::new(socket),
            pending: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn listen(rpc: Rpc, sender: Sender<Message>) {
        thread::spawn(move || {
            let mut buff = [0u8; 4096];
            loop {
                let (n, _) = rpc.socket.recv_from(&mut buff).unwrap();
                let msg: Message = bincode::deserialize(&buff[..n]).unwrap();
                match msg.msg {
                    MessageType::Terminate => {
                        sender.send(msg).unwrap();
                        break;
                    }
                    MessageType::Request(_) => {
                        sender.send(msg.clone()).unwrap();
                    }
                    MessageType::Response(res) => {
                        rpc.clone().response_handler(res, msg.id);
                    }
                }
            }
        });
    }

    fn response_handler(self, response: Response, req_id: String) {
        thread::spawn(move || {
            let mut pending = self.pending.lock().unwrap();
            let sender = pending.get(&req_id);
            if let Some(sender) = sender {
                sender.send(Some(response)).unwrap();
                pending.remove(&req_id);
            }
        });
    }

    pub fn send_request(&self, msg: Message) -> Receiver<Option<Response>> {
        let (tx, rx) = crossbeam_channel::bounded(1);
        let pending = Arc::clone(&self.pending);
        let mut ps = pending.lock().unwrap();
        ps.insert(msg.id.clone(), tx.clone());
        drop(ps);
        self.send_msg(msg.clone())
            .expect("failed to send msg to node");

        let pending = Arc::clone(&self.pending);
        thread::spawn(move || {
            thread::sleep(TIMEOUT);
            if tx.send(None).is_ok() {
                let mut pending = pending.lock().unwrap();
                pending.remove(&msg.id);
            }
        });

        rx
    }

    pub fn send_msg(&self, msg: Message) -> anyhow::Result<()> {
        let encoded_msg = bincode::serialize(&msg).expect("unable to serialize message");
        if self
            .socket
            .send_to(encoded_msg.as_slice(), msg.dist.addr())
            .is_ok()
        {
            anyhow::Ok(())
        } else {
            Err(anyhow::anyhow!("failed to send msg"))
        }
    }
}
