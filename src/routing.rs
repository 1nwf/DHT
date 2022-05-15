use crate::{
    guid::{Distance, GUID, GUID_LEN},
    node::{Location, BUCKET_LEN},
};

#[derive(Clone, Debug)]
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
        let mut closest_nodes = Vec::with_capacity(BUCKET_SIZE);
        let mut idx_copy = idx;

        while closest_nodes.len() != BUCKET_SIZE && idx < self.0.len() - 1 {
            idx += 1;
            let bucket = &self.0[idx];
            for node in &bucket.0 {
                closest_nodes.push(node.clone());
                if closest_nodes.len() == BUCKET_SIZE {
                    break;
                }
            }
        }

        idx = idx_copy;

        while closest_nodes.len() != BUCKET_SIZE && idx > 0 {
            let bucket = &self.0[idx];
            for node in &bucket.0 {
                closest_nodes.push(node.clone());
                if closest_nodes.len() == BUCKET_SIZE {
                    break;
                }
            }
            idx -= 1
        }

        closest_nodes
    }

    pub fn insert(&mut self, id: GUID, node_location: Location) {
        let bucket_idx = Bucket::find_index(&id, &node_location.id);
        let bucket = &mut self.0[bucket_idx];
        if !bucket.is_full() {
            bucket.insert(node_location);
        }
    }

    pub fn print_buckets(&self) {
        for b in &self.0 {
            if b.0.len() > 0 {
                for l in &b.0 {
                    println!("{:?}", l);
                }
            }
        }
        println!("------------------------------");
    }

    pub fn remove(&mut self, id: &GUID, node_id: &GUID) {
        let bucket_idx = Bucket::find_index(id, node_id);
        let bucket = &mut self.0[bucket_idx];
        let node_idx = bucket.0.iter().position(|r| r.id == node_id.clone());
        if let Some(idx) = node_idx {
            bucket.0.remove(idx);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Bucket(pub Vec<Location>);

impl Bucket {
    fn new() -> Self {
        Self(Vec::with_capacity(BUCKET_SIZE))
    }

    pub fn find_index(id: &GUID, node_id: &GUID) -> usize {
        let dist = node_id.distance_from(id);
        let diff = dist.distance_from(&node_id.0);

        for i in 0..GUID_LEN {
            for j in (0..8).rev() {
                let bit = diff[i] & (1 << j);
                if bit != 0 {
                    return 8 * i + j;
                }
            }
        }

        GUID_LEN * 8 - 1
    }

    fn insert(&mut self, node: Location) {
        let node_idx = self.0.iter().position(|l| l.id == node.id);
        if let Some(idx) = node_idx {
            self.0.remove(idx);
        }
        self.0.push(node);
    }

    fn is_full(&self) -> bool {
        self.0.len() >= BUCKET_SIZE
    }
}
