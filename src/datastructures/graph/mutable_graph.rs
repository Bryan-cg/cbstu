use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use log::{trace, warn};
use crate::algorithms::min_bottleneck_spanning_tree::camerini::MBST;
use crate::algorithms::min_sum_spanning_tree::kruskal::{CalculationType, ConnectionType, Kruskal};
use crate::datastructures::graph::edge::Edge;
use crate::datastructures::graph::node::Node;

/// Graph with immutable nodes and mutable edges.
pub struct MutableGraph {
    nodes: Rc<Vec<Rc<Node>>>,
    edges: Vec<Rc<RefCell<Edge>>>,
}

impl MutableGraph {
    pub fn new(nodes: Rc<Vec<Rc<Node>>>, edges: Vec<Rc<RefCell<Edge>>>) -> MutableGraph {
        MutableGraph {
            nodes,
            edges,
        }
    }

    pub fn nodes(&self) -> &Vec<Rc<Node>> {
        &self.nodes
    }

    pub fn edges(&self) -> &Vec<Rc<RefCell<Edge>>> {
        &self.edges
    }

    //only for printing out edges
    pub fn edges_borrowed(&self) -> Vec<Rc<Edge>> {
        let mut res = Vec::new();
        self.edges.iter().for_each(|edge| {
            res.push(Rc::new(edge.borrow().clone()));
        });
        res
    }

    pub fn edges_mut(&mut self) -> &mut Vec<Rc<RefCell<Edge>>> {
        &mut self.edges
    }

    pub fn edges_copy(&self) -> Vec<Rc<RefCell<Edge>>> {
        self.edges.clone()
    }

    pub fn nodes_copy(&self) -> Rc<Vec<Rc<Node>>> {
        Rc::clone(&self.nodes)
    }

    pub fn min_sum_st(&mut self, calculation_type: CalculationType) -> (Option<MutableGraph>, f64, f64) {
        Kruskal::run_mutable(self, calculation_type, None)
    }

    pub fn mcst(&mut self, calculation_type: CalculationType) -> (ConnectionType, MutableGraph, f64, f64) {
        Kruskal::run(self, calculation_type)
    }

    pub fn min_sum_early_detection(&mut self, calculation_type: CalculationType, budget: Option<f64>) -> (Option<MutableGraph>, f64, f64) {
        Kruskal::run_mutable(self, calculation_type, budget)
    }

    pub fn min_bot_st(&mut self) -> (Option<MutableGraph>, f64) {
        MBST::run_mutable(self)
    }

    pub fn calculate_total_cost(&self) -> f64 {
        self.edges.iter().fold(0.0, |acc, edge| acc + edge.borrow().get_cost())
    }

    pub fn get_edges_weight_lower_or_eq_than(&self, weight: f64) -> MutableGraph {
        let mut edges = Vec::with_capacity(self.edges.len());
        self.edges.iter().for_each(|edge| {
            if edge.borrow().get_weight() <= weight {
                edges.push(Rc::clone(edge));
            }
        });
        MutableGraph { nodes: Rc::clone(&self.nodes), edges }
    }

    pub fn get_edges_weight_bigger_than(&self, weight: f64) -> MutableGraph {
        let mut res_edges = Vec::new();
        self.edges.iter().for_each(|edge| {
            if edge.borrow().get_weight() > weight {
                res_edges.push(Rc::clone(edge));
            }
        });
        // clones only the pointer to the nodes
        MutableGraph { nodes: Rc::clone(&self.nodes), edges: res_edges }
    }

    fn adj_edges(&self, node_id: usize) -> Vec<Rc<RefCell<Edge>>> {
        let mut edges = Vec::new();
        self.edges.iter().for_each(|edge| {
            let (u, v) = edge.borrow().endpoints();
            if u == node_id || v == node_id {
                edges.push(Rc::clone(edge));
            }
        });
        edges
    }

    pub fn is_connected_graph(&self) -> bool {
        //check if all nodes are connected
        trace!("Checking if graph is connected");
        let mut visited = vec![false; self.nodes.len()];
        let mut stack = Vec::new();
        stack.push(0);
        let mut iter = 0;
        while !stack.is_empty() {
            let node_id = stack.pop().unwrap();
            iter += 1;
            if iter % 1000 == 0 {
                trace!("iter: {}", iter);
            }
            if !visited[node_id] {
                visited[node_id] = true;
                let adj_edges = self.adj_edges(node_id);
                adj_edges.iter().for_each(|edge| {
                    let (u, v) = edge.borrow().endpoints();
                    if !visited[u] {
                        stack.push(u);
                    }
                    if !visited[v] {
                        stack.push(v);
                    }
                });
            }
        }
        visited.iter().all(|&x| x)
    }

    ///Slow check if the graph is a spanning tree and fully connected, only use for debugging
    pub fn is_spanning_tree(&self) -> bool {
        trace!("Checking if graph is a spanning tree");
        if self.edges.len() != self.nodes.len() - 1 {
            return false;
        }
        //check if all nodes are visited [DFS]
        let mut visited_nodes = vec![false; self.nodes.len()];
        let mut stack = Vec::new();
        stack.push(0);
        let max_iterations = self.nodes.len();
        let mut iterations = 0;
        while !stack.is_empty() {
            iterations += 1;
            if iterations % 5000 == 0 {
                trace!("DFS progress: {}/{}", iterations, max_iterations);
            }
            let node = stack.pop().unwrap();
            visited_nodes[node] = true;
            let edges = self.adj_edges(node);
            for edge in edges {
                let (node1, node2) = edge.borrow().endpoints();
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
        trace!("Graph is a spanning tree");
        true
    }

    pub fn inverse_weights(&mut self) {
        self.edges.iter().for_each(|edge| {
            edge.borrow_mut().inverse_weights();
        });
    }

    //Dont use this function, it is only for debugging
    pub fn has_upgraded_equivalent(&self, u: usize, v: usize) -> bool {
        for edge in self.adj_edges(u) {
            if edge.borrow().endpoints().0 == v || edge.borrow().endpoints().1 == v {
                if edge.borrow().is_upgraded() { return true; }
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::datastructures::graph::edge::Edge;
    use crate::datastructures::graph::mutable_graph::MutableGraph;
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

        edges.push(Rc::new(RefCell::new(Edge::new(0, 1).weight(1.0))));
        edges.push(Rc::new(RefCell::new(Edge::new(0, 2).weight(2.0))));
        edges.push(Rc::new(RefCell::new(Edge::new(0, 3).weight(3.0))));
        edges.push(Rc::new(RefCell::new(Edge::new(1, 2).weight(4.0))));
        edges.push(Rc::new(RefCell::new(Edge::new(1, 3).weight(5.0))));
        edges.push(Rc::new(RefCell::new(Edge::new(2, 3).weight(6.0))));
        let graph = MutableGraph::new(Rc::new(nodes), edges);
        let graph2 = graph.get_edges_weight_lower_or_eq_than(3.0);
        assert_eq!(graph2.edges().len(), 3);
        //check number of pointers in rc
        assert_eq!(Rc::strong_count(&graph2.edges()[0]), 2);
    }
}