pub struct DSU {
    parent: Vec<usize>,
    size: Vec<i64>,
}

impl DSU {
    pub fn new(len: usize) -> Self {
        Self {
            parent: (0..len).collect(),
            size: vec![1; len],
        }
    }

    pub fn get_size(&self, node: usize) -> i64 {
        self.size[node]
    }

    pub fn get_parent(&mut self, node: usize) -> usize {
        if self.parent[node] == node {
            return node;
        }

        self.parent[node] = self.get_parent(self.parent[node]);
        self.parent[node]
    }

    pub fn merge(&mut self, node1: usize, node2: usize) -> bool {
        let parent1 = self.get_parent(node1);
        let parent2 = self.get_parent(node2);

        if parent1 == parent2 {
            return false;
        }

        let size1 = self.size[parent1];
        let size2 = self.size[parent2];

        if size1 > size2 {
            self.parent[parent2] = parent1;
            self.size[parent1] += size2;
        } else {
            self.parent[parent1] = parent2;
            self.size[parent2] += size1;
        }

        true
    }
}
