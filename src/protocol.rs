use crate::{guid::GUID, node::Location};

pub trait Protocol {
    fn ping(&self, node_addr: String) -> anyhow::Result<()>;
    fn store(key: String, val: String) -> anyhow::Result<()>;
    fn find_node(id: GUID) -> Vec<Location>;
    fn find_value(key: String) -> Option<String>;
    fn get_value(key: String) -> Option<String>;
}
