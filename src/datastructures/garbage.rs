use std::rc::Rc;
use crate::datastructures::graph::mutable_graph::MutableGraph;

/// Simple datastructure to cache graphs that can be cleaned up at a later stage.
pub struct Garbage {
    trash: Vec<Rc<MutableGraph>>,
}

impl Garbage {
    pub fn new() -> Garbage {
        Garbage {
            trash: Vec::new(),
        }
    }
    pub fn default() -> Garbage {
        Garbage {
            trash: Vec::with_capacity(0),
        }
    }
    pub fn add(&mut self, graph: Rc<MutableGraph>) {
        self.trash.push(graph);
    }
    pub fn clear(&mut self) {
        self.trash.clear();
    }
    pub fn len(&self) -> usize {
        self.trash.len()
    }
}