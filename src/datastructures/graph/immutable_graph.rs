use std::rc::Rc;
use log::{trace, warn};
use crate::algorithms::min_bottleneck_spanning_tree::camerini::MBST;
use crate::algorithms::min_sum_spanning_tree::kruskal::{CalculationType, Kruskal};
use crate::datastructures::graph::edge::Edge;
use crate::datastructures::graph::node::Node;
use crate::print_edges;

/// Graph with immutable nodes and edges. The edges can get a different order in the vector.
pub struct ImmutableGraph {
    nodes: Rc<Vec<Rc<Node>>>,
    edges: Vec<Rc<Edge>>,
}

impl ImmutableGraph {
    pub fn new(nodes: Rc<Vec<Rc<Node>>>, edges: Vec<Rc<Edge>>) -> ImmutableGraph {
        ImmutableGraph {
            nodes,
            edges,
        }
    }

    pub fn min_sum_st(&mut self, calculation_type: CalculationType) -> (Option<ImmutableGraph>, f64, f64) {
        Kruskal::run(self, calculation_type)
    }

    pub fn min_bot_st(&mut self) -> (Option<ImmutableGraph>, f64) {
        MBST::run(self)
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

    pub fn nodes_copy(&self) -> Rc<Vec<Rc<Node>>> {
        Rc::clone(&self.nodes)
    }

    pub fn get_node(&self, id: usize) -> Option<&Rc<Node>> {
        self.nodes.iter().find(|node| node.id() == id)
    }
    pub fn get_edges_weight_lower_or_eq_than(&self, weight: f64) -> ImmutableGraph {
        let mut res_edges = Vec::new();
        self.edges.iter().for_each(|edge| {
            if edge.get_weight() <= weight {
                res_edges.push(Rc::clone(edge));
            }
        });
        // clones only the pointers to the nodes
        ImmutableGraph { nodes: self.nodes.clone(), edges: res_edges }
    }
    pub fn get_edges_weight_bigger_than(&self, weight: f64) -> ImmutableGraph {
        let mut res_edges = Vec::new();
        self.edges.iter().for_each(|edge| {
            if edge.get_weight() > weight {
                res_edges.push(Rc::clone(edge));
            }
        });
        // clones only the pointers to the nodes
        ImmutableGraph { nodes: self.nodes.clone(), edges: res_edges }
    }
    ///Slow check if the graph is a spanning tree and fully connected, only use for debugging
    pub fn is_spanning_tree(&self) -> bool {
        if self.edges.len() != self.nodes.len() - 1 {
            return false;
        }
        //check if all nodes are visited [DFS]
        let mut visited_nodes = vec![false; self.nodes.len()];
        let mut stack = Vec::new();
        stack.push(0);
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            visited_nodes[node] = true;
            let edges = self.get_edges_by_node(node);
            for edge in edges {
                let (node1, node2) = edge.endpoints();
                if !visited_nodes[node1] {
                    stack.push(node1);
                }
                if !visited_nodes[node2] {
                    stack.push(node2);
                }
            }
        }
        for (i, visited) in visited_nodes.iter().enumerate() {
            if !visited {
                warn!("Node {} not visited in ST", i);
                return false;
            }
        }
        true
    }
    fn get_edges_by_node(&self, node_id: usize) -> Vec<Rc<Edge>> {
        let mut res = Vec::new();
        for edge in &self.edges {
            let (v, w) = edge.endpoints();
            if v == node_id|| w == node_id {
                res.push(Rc::clone(edge));
            }
        }
        res
    }
    pub fn calculate_total_weight(&self) -> f64 {
        self.edges.iter().fold(0.0, |acc, edge| acc + edge.get_weight())
    }
    pub fn calculate_total_cost(&self) -> f64 {
        self.edges.iter().fold(0.0, |acc, edge| acc + edge.get_cost())
    }
    pub fn negative_weights(&self) -> ImmutableGraph {
        let mut res_edges = Vec::new();
        self.edges.iter().for_each(|edge| {
            let (v, w) = edge.endpoints();
            res_edges.push(Rc::new(Edge::new(v, w)
                .weight(-edge.get_weight())
                .upgraded_weight(-edge.get_upgraded_weight())
                .cost(edge.get_cost())));
        });
        ImmutableGraph { nodes: self.nodes.clone(), edges: res_edges }
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
        let graph = ImmutableGraph::new(Rc::new(nodes), edges);
        let graph2 = graph.get_edges_weight_lower_or_eq_than(3.0);
        assert_eq!(graph2.edges().len(), 3);
        //check number of pointers in rc
        assert_eq!(Rc::strong_count(&graph2.edges()[0]), 2);
    }
}
