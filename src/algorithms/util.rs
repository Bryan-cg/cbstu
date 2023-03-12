use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use array_tool::vec::{Union, Uniq};
use fxhash::FxHashSet;
use log::{debug, trace};
use crate::algorithms::min_sum_spanning_tree::kruskal::CalculationType;
use crate::datastructures::graph::edge::Edge;
use crate::datastructures::graph::mutable_graph::MutableGraph;

///macro to print edges of ```Vec<Rc<RefCell<Edge>>>```
#[macro_export]
macro_rules! print_edges {
    ($edges:expr) => {
        $edges.iter().for_each(|edge| {
            println!("{} - {}, {} - {}, {}, {}", edge.borrow().endpoints().0, edge.borrow().endpoints().1, edge.borrow().original_endpoints().0, edge.borrow().original_endpoints().1, edge.borrow().get_weight(), edge.borrow().is_upgraded());
        });
    };
}

pub enum PivotResult {
    Feasible((MutableGraph, f64, f64)),
    Infeasible,
}

pub enum PivotChecked {
    Feasible((MutableGraph, f64, f64)),
    Infeasible(MutableGraph),
}

pub struct Util();

impl Util {
    #[inline]
    /// Creates a new graph with the same nodes, but each edge is duplicated with its original weight (cost 0) and upgraded weight (upgrade cost).
    pub fn duplicate_edges(graph: &MutableGraph) -> MutableGraph {
        let mut edges = Vec::new();
        for edge in graph.edges() {
            let (u, v) = edge.borrow().endpoints();
            edges.push(Rc::new(RefCell::new(Edge::new(u, v).weight(edge.borrow().get_weight()).cost(0.0).upgraded(false))));
            edges.push(Rc::new(RefCell::new(Edge::new(u, v).weight(edge.borrow().get_upgraded_weight()).cost(edge.borrow().get_cost()).upgraded(true).or_weight(edge.borrow().get_weight()))));
        }
        MutableGraph::new(graph.nodes_copy(), edges)
    }

    #[inline]
    /// Updates bottleneck to bigger/smaller value according to inverse.
    pub fn update_bottleneck(bottleneck: f64, edge: &Rc<RefCell<Edge>>, inverse: bool) -> f64 {
        let mut bottleneck = bottleneck;
        if inverse {
            if edge.borrow().get_weight() > bottleneck {
                bottleneck = edge.borrow().get_weight();
            }
        } else if edge.borrow().get_weight() < bottleneck {
            bottleneck = edge.borrow().get_weight();
        }
        bottleneck
    }

    #[inline]
    ///union of 2 list of edges without duplicates
    pub fn union_edges(edges1: &Vec<Rc<RefCell<Edge>>>, edges2: &Vec<Rc<RefCell<Edge>>>) -> Vec<Rc<RefCell<Edge>>> {
        let mut edges = FxHashSet::default();
        edges1.iter().for_each(|edge| {
            edges.insert(edge.borrow().clone());
        });
        edges2.iter().for_each(|edge| {
            edges.insert(edge.borrow().clone());
        });
        edges.into_iter().map(|edge| Rc::new(RefCell::new(edge))).collect()
    }

    #[inline]
    ///Return unique list of weights with weight bigger then lower-bound and smaller then or equal to upperbound
    pub fn unique_weight_list(edges: &[Rc<RefCell<Edge>>], lower_bound: f64, upper_bound: f64) -> Vec<f64> {
        let mut weights = FxHashSet::default();
        edges.iter().for_each(|edge| {
            if edge.borrow().get_weight() > lower_bound && edge.borrow().get_weight() <= upper_bound {
                weights.insert(edge.borrow().get_weight() as i64);
            }
        });
        weights.into_iter().map(|weight| weight as f64).collect()
    }

    #[inline]
    ///Return unique list of weights with weight bigger then lower-bound and smaller then or equal to upperbound
    pub fn unique_weight_list_above_or_eq(edges: &[Rc<RefCell<Edge>>], threshold: f64) -> Vec<f64> {
        let mut weights_set = FxHashSet::default();
        edges.iter().for_each(|edge| {
            if edge.borrow().get_weight() >= threshold {
                weights_set.insert(edge.borrow().get_weight() as i64);
            }
        });
        weights_set.into_iter().map(|weight| weight as f64).collect()
    }

    #[inline]
    ///Return vector slice with weight bigger then lower-bound en smaller then or equal to upper bound
    pub fn relevant_slice(weights: &[f64], lower_bound: f64, upper_bound: f64) -> Vec<f64> {
        weights.iter().filter(|&&x| x > lower_bound && x <= upper_bound).cloned().collect()
    }

    #[inline]
    /// Check if graph with edge weights <= pivot_weight is feasible
    pub fn check_pivot(graph: &MutableGraph, pivot_weight: f64, budget: f64) -> PivotResult {
        let mut graph_below_pivot = graph.smaller_or_eq_than(pivot_weight);
        let (op_mst, cost, bottleneck) = graph_below_pivot.mst(CalculationType::Cost);
        match op_mst {
            Some(st) => {
                match cost {
                    cost if cost <= budget => PivotResult::Feasible((st, cost, bottleneck)),
                    _ => PivotResult::Infeasible
                }
            }
            None => PivotResult::Infeasible
        }
    }
}