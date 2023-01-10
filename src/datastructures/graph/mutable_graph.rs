use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::algorithms::min_bottleneck_spanning_tree::camerini::MBST;
use crate::algorithms::min_sum_spanning_tree::kruskal::{CalculationType, Kruskal};
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
        Kruskal::run_mutable(self, calculation_type)
    }

    pub fn min_bot_st(&mut self) -> (Option<MutableGraph>, f64) {
        MBST::run_mutable(self)
    }

    pub fn calculate_total_cost(&self) -> f64 {
        self.edges.iter().fold(0.0, |acc, edge| acc + edge.borrow().get_cost())
    }

    pub fn get_edges_weight_lower_or_eq_than(&self, weight: f64) -> MutableGraph {
        let mut edges = Vec::new();
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
        // clones only the pointers to the nodes
        MutableGraph { nodes: Rc::clone(&self.nodes), edges: res_edges }
    }
}