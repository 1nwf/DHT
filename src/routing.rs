use crate::{
    guid::{Distance, GUID, GUID_LEN},
    node::{Location, BUCKET_LEN},
    util::leading_zeros,
};

#[derive(Clone, Debug)]
pub struct RoutingTable(pub Vec<Bucket>);
const BUCKET_SIZE: usize = 20;
impl RoutingTable {
    pub fn new() -> Self {
        let mut buckets = Vec::new();

        for _ in 0..BUCKET_LEN {
            buckets.push(Bucket::new())
        }

        Self(buckets)
    }

    pub fn nearest_nodes_to_id(&self, node_id: &GUID, id: &GUID) -> Vec<Location> {
        let idx = Bucket::find_index(node_id, id);
        let mut closest_nodes = Vec::new();
        closest_nodes.extend(self.0[idx].0.clone());

        if closest_nodes.len() < BUCKET_SIZE {
            for i in (idx + 1)..self.0.len() {
                closest_nodes.extend(self.0[i].0.clone())
            }
        }

        if closest_nodes.len() < BUCKET_SIZE {
            for i in 0..idx {
                closest_nodes.extend(self.0[i].0.clone())
            }
        }

        closest_nodes.sort();
        closest_nodes.truncate(BUCKET_SIZE);

        closest_nodes
    }

    pub fn insert(&mut self, id: GUID, node_location: Location) {
        let bucket_idx = Bucket::find_index(&id, &node_location.id);
        let bucket = &mut self.0[bucket_idx];
        if !bucket.is_full() {
            if bucket.0.iter().any(|x| *x == node_location) {
                return;
            }
            bucket.insert(node_location);
        }
    }

    pub fn remove(&mut self, id: &GUID, node_id: &GUID) {
        let bucket_idx = Bucket::find_index(id, node_id);
        let bucket = &mut self.0[bucket_idx];
        let node_idx = bucket.0.iter().position(|r| r.id == *node_id);
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
        let dist = Distance::calc(id, node_id);

        for (idx, val) in dist.0.iter().enumerate() {
            if *val != 0 {
                return idx * 8 + leading_zeros(*val);
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

#[test]
fn test_bucket_idx() {
    let id1 = GUID([0; GUID_LEN]);
    let mut id2 = GUID([0; GUID_LEN]);
    assert_eq!(Bucket::find_index(&id1, &id2), 255);

    id2 = GUID([1; GUID_LEN]);
    assert_eq!(Bucket::find_index(&id1, &id2), 7);

    id2 = GUID([128; GUID_LEN]);
    assert_eq!(Bucket::find_index(&id1, &id2), 0);

    id2 = GUID([4; GUID_LEN]);
    assert_eq!(Bucket::find_index(&id1, &id2), 5);

    id2 = GUID([8; GUID_LEN]);
    assert_eq!(Bucket::find_index(&id1, &id2), 4);
}
