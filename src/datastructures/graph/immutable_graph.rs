use std::rc::Rc;
use crate::datastructures::graph::edge::Edge;
use crate::datastructures::graph::node::Node;

/// Graph with immutable nodes and edges
pub struct ImmutableGraph {
    nodes: Vec<Rc<Node>>,
    edges: Vec<Rc<Edge>>,
}

impl ImmutableGraph {
    pub fn new(nodes: Vec<Rc<Node>>, edges: Vec<Rc<Edge>>) -> ImmutableGraph {
        ImmutableGraph {
            nodes,
            edges,
        }
    }

    pub fn nodes(&self) -> &Vec<Rc<Node>> {
        &self.nodes
    }

    pub fn edges(&self) -> &Vec<Rc<Edge>> {
        &self.edges
    }

    pub fn edges_mut(&mut self) -> &mut Vec<Rc<Edge>> {
        &mut self.edges
    }

    pub fn edges_copy(&self) -> Vec<Rc<Edge>> {
        self.edges.clone()
    }

    pub fn nodes_copy(&self) -> Vec<Rc<Node>> {
        self.nodes.clone()
    }

    pub fn get_node(&self, id: usize) -> Option<&Rc<Node>> {
        self.nodes.iter().find(|node| node.id() == id)
    }
    pub fn get_edges_weight_lower_or_eq_than(&self, weight: f64) -> ImmutableGraph {
        let mut res_edges = Vec::new();
        self.edges.iter().for_each(|edge|
            {
                if edge.get_weight() <= weight {
                    res_edges.push(Rc::clone(edge));
                }
            });
        // clones only the pointers to the nodes
        ImmutableGraph { nodes: self.nodes.clone(), edges: res_edges }
    }
    pub fn is_spanning_tree(&self) -> bool {
        self.edges.len() == self.nodes.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::datastructures::graph::edge::Edge;
    use crate::datastructures::graph::immutable_graph::ImmutableGraph;
    use crate::datastructures::graph::node::Node;

    #[test]
    fn test_get_edges_weight_lower_or_eq_than() {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let node1 = Rc::new(Node::new(0, 0.0, 0.0));
        let node2 = Rc::new(Node::new(1, 0.0, 0.0));
        let node3 = Rc::new(Node::new(2, 0.0, 0.0));
        let node4 = Rc::new(Node::new(3, 0.0, 0.0));
        nodes.push(Rc::clone(&node1));
        nodes.push(Rc::clone(&node2));
        nodes.push(Rc::clone(&node3));
        nodes.push(Rc::clone(&node4));

        edges.push(Rc::new(Edge::new(0, 1).weight(1.0)));
        edges.push(Rc::new(Edge::new(0, 2).weight(2.0)));
        edges.push(Rc::new(Edge::new(0, 3).weight(3.0)));
        edges.push(Rc::new(Edge::new(1, 2).weight(4.0)));
        edges.push(Rc::new(Edge::new(1, 3).weight(5.0)));
        edges.push(Rc::new(Edge::new(2, 3).weight(6.0)));
        let graph = ImmutableGraph::new(nodes, edges);
        let graph2 = graph.get_edges_weight_lower_or_eq_than(3.0);
        assert_eq!(graph2.edges().len(), 3);
        //check number of pointers in rc
        assert_eq!(Rc::strong_count(&graph2.edges()[0]), 2);
    }
}
