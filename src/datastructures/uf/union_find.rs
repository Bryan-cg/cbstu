pub struct UF {
    id: Vec<usize>,
    size: Vec<usize>,
    count: i32,
}

impl UF {
    pub fn new(n: i32) -> UF {
        if n < 0 { panic!("Number of nodes must be non-negative"); }
        let mut id = Vec::new();
        let mut size = Vec::new();
        for i in 0..n {
            id.push(i as usize);
            size.push(1);
        }
        UF {
            id,
            size,
            count: n,
        }
    }
    pub fn find(&mut self, p: usize) -> usize {
        let mut p = p;
        while p != self.id[p] {
            self.id[p] = self.id[self.id[p]];
            p = self.id[p];
        }
        p
    }
    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }
    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.find(p);
        let root_q = self.find(q);
        if root_p == root_q { return; }
        if self.size[root_p] < self.size[root_q] {
            self.id[root_p] = root_q;
            self.size[root_q] += self.size[root_p];
        } else {
            self.id[root_q] = root_p;
            self.size[root_p] += self.size[root_q];
        }
        self.count -= 1;
    }

    pub fn count(&self) -> i32 {
        self.count
    }
}

#[test]
fn test_uf() {
    let mut uf = UF::new(10);
    assert_eq!(uf.count, 10);
    assert_eq!(uf.connected(1, 2), false);
    uf.union(1, 2);
    assert_eq!(uf.connected(1, 2), true);
    assert_eq!(uf.count, 9);
    assert_eq!(uf.connected(1, 3), false);
    uf.union(1, 3);
    assert_eq!(uf.connected(1, 3), true);
    assert_eq!(uf.count, 8);
    assert_eq!(uf.connected(1, 4), false);
    uf.union(1, 4);
    assert_eq!(uf.connected(1, 4), true);
    assert_eq!(uf.count, 7);
    assert_eq!(uf.connected(1, 5), false);
    uf.union(1, 5);
    assert_eq!(uf.connected(1, 5), true);
    assert_eq!(uf.count, 6);
    assert_eq!(uf.connected(1, 6), false);
    uf.union(1, 6);
    assert_eq!(uf.connected(1, 6), true);
    assert_eq!(uf.count, 5);
    assert_eq!(uf.connected(1, 7), false);
    uf.union(1, 7);
    assert_eq!(uf.connected(1, 7), true);
    assert_eq!(uf.count, 4);
    assert_eq!(uf.connected(1, 8), false);
    uf.union(1, 8);
    assert_eq!(uf.connected(1, 8), true);
    assert_eq!(uf.count, 3);
    assert_eq!(uf.connected(1, 9), false);
    uf.union(1, 9);
    assert_eq!(uf.connected(1, 9), true);
    assert_eq!(uf.count, 2);
    assert_eq!(uf.connected(1, 0), false);
    uf.union(1, 0);
    assert_eq!(uf.connected(1, 0), true);
    assert_eq!(uf.count, 1);
}

#[test]
fn test_parent() {
    let mut uf = UF::new(5);
    uf.union(0, 1);
    uf.union(1, 2);
    assert_eq!(uf.find(0), uf.find(2));
}