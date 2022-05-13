use crate::{guid::GUID, message::FindValue, node::Location};

pub trait Protocol {
    fn ping(&self, dist: Location) -> anyhow::Result<()>;
    fn store(&mut self, key: String, val: String, dist: Location) -> anyhow::Result<()>;
    fn find_node(&self, id: GUID, dist: Location) -> Option<Vec<Location>>;
    fn find_value(&self, key: String, dist: Location) -> FindValue;
}
