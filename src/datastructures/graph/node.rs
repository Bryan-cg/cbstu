#[derive(Debug, Clone)]
pub struct Node {
    id: usize,
    x: f64,
    y: f64,
}

impl Node {
    pub fn new(id: usize, x: f64, y: f64) -> Node {
        Node { id, x, y }
    }
    pub fn default(id: usize) -> Node {
        Node { id, x: 0.0, y: 0.0 }
    }
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Node {}