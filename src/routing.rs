use crate::{
    guid::{Distance, GUID, GUID_LEN},
    node::{self, Location, BUCKET_LEN},
};

#[derive(Clone)]
pub struct RoutingTable(pub Vec<Bucket>);

const BUCKET_SIZE: usize = 20;
impl RoutingTable {
    pub fn new() -> Self {
        let mut buckets = Vec::new();

        for i in 0..BUCKET_LEN {
            buckets.push(Bucket::new())
        }

        Self(buckets)
    }

    pub fn nearest_nodes_to_id(&self, node_id: &GUID, id: &GUID) -> Vec<Location> {
        let mut idx = Bucket::find_index(node_id, id);
        let mut bucket = &self.0[idx];
        let mut closest_nodes = Vec::with_capacity(BUCKET_SIZE);
        let mut idx_copy = idx;

        while closest_nodes.len() != BUCKET_SIZE {
            bucket = &self.0[idx];
            for node in &bucket.0 {
                closest_nodes.push(node.clone());
            }
            if idx < self.0.len() - 1 {
                idx += 1
            } else {
                idx_copy -= 1;
                idx = idx_copy
            }
        }

        closest_nodes
    }
}

#[derive(Clone)]
pub struct Bucket(pub Vec<Location>);

impl Bucket {
    fn new() -> Self {
        Self(Vec::with_capacity(BUCKET_SIZE))
    }

    fn find_index(id: &GUID, node_id: &GUID) -> usize {
        let dist = node_id.distance_from(&id);
        let diff = dist.distance_from(&node_id.0);

        for i in 0..GUID_LEN {
            for j in (0..8).rev() {
                let bit = diff[i] & (1 << j);
                if bit != 0 {
                    return 8 * i - j;
                }
            }
        }

        GUID_LEN * 8 - 1
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

