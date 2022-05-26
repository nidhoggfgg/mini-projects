pub trait Find {
    fn find(&self, v: usize) -> usize;
    fn union(&mut self, p: usize, q: usize);
    fn count(&self) -> u32;
}

pub struct UnionFind {
    ids: Vec<usize>,
    sz: Vec<usize>,
    count: u32,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut ids = Vec::with_capacity(n);
        let sz = vec![1; n];

        for i in 0..n {
            ids.push(i);
        }

        UnionFind {
            ids,
            sz,
            count: n as u32,
        }
    }
}

impl Find for UnionFind {
    fn find(&self, v: usize) -> usize {
        let mut v = v;
        while v != self.ids[v] {
            v = self.ids[v];
        }
        v
    }

    fn union(&mut self, p: usize, q: usize) {
        let p_root = self.find(p);
        let q_root = self.find(q);

        if p_root == q_root {
            return;
        }

        if self.sz[p_root] > self.sz[q_root] {
            self.ids[q_root] = p_root;
            self.sz[p_root] += self.sz[q_root];
        } else {
            self.ids[p_root] = q_root;
            self.sz[q_root] += self.sz[p_root];
        }
    }

    #[inline]
    fn count(&self) -> u32 {
        self.count
    }
}
