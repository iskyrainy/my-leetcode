#[derive(Clone, Copy)]
struct Node {
    key: i32,
    val: i32,
    prev: usize,
    next: usize,
}

pub struct LRUCache {
    capacity: usize,
    nodes: Vec<Node>,
    map: std::collections::HashMap<i32, usize>,
    free_nodes: Vec<usize>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let cap = capacity as usize;
        let mut nodes = vec![Node { key: 0, val: 0, prev: 0, next: 0 }; cap + 2];
        
        nodes[0].next = 1;
        nodes[1].prev = 0;

        Self {
            capacity: cap,
            nodes,
            map: std::collections::HashMap::with_capacity(cap),
            free_nodes: (2..cap + 2).collect(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&idx) = self.map.get(&key) {
            self.detach(idx);
            self.attach_to_head(idx);
            return self.nodes[idx].val;
        }
        -1
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&idx) = self.map.get(&key) {
            self.nodes[idx].val = value;
            self.detach(idx);
            self.attach_to_head(idx);
        } else {
            let idx = if let Some(free_idx) = self.free_nodes.pop() {
                free_idx
            } else {
                let last_idx = self.nodes[1].prev;
                self.map.remove(&self.nodes[last_idx].key);
                self.detach(last_idx);
                last_idx
            };

            self.nodes[idx].key = key;
            self.nodes[idx].val = value;
            self.map.insert(key, idx);
            self.attach_to_head(idx);
        }
    }

    #[inline(always)]
    fn detach(&mut self, idx: usize) {
        let p = self.nodes[idx].prev;
        let n = self.nodes[idx].next;
        self.nodes[p].next = n;
        self.nodes[n].prev = p;
    }

    #[inline(always)]
    fn attach_to_head(&mut self, idx: usize) {
        let old_head_next = self.nodes[0].next;
        
        self.nodes[idx].next = old_head_next;
        self.nodes[idx].prev = 0;
        
        self.nodes[0].next = idx;
        self.nodes[old_head_next].prev = idx;
    }
}
