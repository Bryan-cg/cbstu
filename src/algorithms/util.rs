use std::cell::RefCell;
use std::rc::Rc;
use array_tool::vec::{Union, Uniq};
use crate::datastructures::graph::edge::Edge;
use crate::datastructures::graph::immutable_graph::ImmutableGraph;
use crate::datastructures::graph::mutable_graph::MutableGraph;

///macro to print edges of Vec<Rc<Edge>>
#[macro_export]
macro_rules! print_edges {
    ($edges:expr) => {
        $edges.iter().for_each(|edge| {
            println!("{} - {}, {}", edge.borrow().endpoints().0, edge.borrow().endpoints().1, edge.borrow().get_weight());
        });
    };
}

pub enum PivotResult {
    Feasible((ImmutableGraph, f64, f64)),
    Infeasible,
}

pub enum PivotResultMut {
    Feasible((MutableGraph, f64, f64)),
    Infeasible,
}

pub struct Util();

impl Util {
    #[inline]
    ///Creates a new graph with the same nodes, but each edge is duplicated with its original weight (cost 0) and upgraded weight (upgrade cost).
    pub fn duplicate_edges(graph: &ImmutableGraph) -> ImmutableGraph {
        let mut edges = Vec::new();
        for edge in graph.edges() {
            let (u, v) = edge.endpoints();
            edges.push(Rc::new(Edge::new(u, v).weight(edge.get_weight()).cost(0.0).upgraded(false)));
            edges.push(Rc::new(Edge::new(u, v).weight(edge.get_upgraded_weight()).cost(edge.get_cost()).upgraded(true)));
        }
        ImmutableGraph::new(graph.nodes_copy(), edges)
    }

    #[inline]
    ///Creates a new graph with the same nodes, but each edge is duplicated with its original weight (cost 0) and upgraded weight (upgrade cost).
    pub fn duplicate_edges_mut(graph: &MutableGraph) -> MutableGraph {
        let mut edges = Vec::new();
        for edge in graph.edges() {
            let (u, v) = edge.borrow().endpoints();
            edges.push(Rc::new(RefCell::new(Edge::new(u, v).weight(edge.borrow().get_weight()).cost(0.0).upgraded(false))));
            edges.push(Rc::new(RefCell::new(Edge::new(u, v).weight(edge.borrow().get_upgraded_weight()).cost(edge.borrow().get_cost()).upgraded(true))));
        }
        MutableGraph::new(graph.nodes_copy(), edges)
    }


    #[inline]
    pub fn duplicate_only_upgraded(graph: &ImmutableGraph) -> ImmutableGraph {
        let mut edges = Vec::new();
        for edge in graph.edges() {
            let (u, v) = edge.endpoints();
            edges.push(Rc::new(Edge::new(u, v).weight(edge.get_upgraded_weight()).cost(edge.get_cost()).upgraded(true)));
        }
        ImmutableGraph::new(graph.nodes_copy(), edges)
    }

    #[inline]
    pub fn update_bottleneck(bottleneck: f64, edge: &Rc<Edge>, inverse: bool) -> f64 {
        let mut bottleneck = bottleneck;
        if inverse {
            if edge.get_weight() > bottleneck {
                bottleneck = edge.get_weight();
            }
        } else if edge.get_weight() < bottleneck {
            bottleneck = edge.get_weight();
        }
        bottleneck
    }

    #[inline]
    pub fn update_bottleneck_mut(bottleneck: f64, edge: &Rc<RefCell<Edge>>, inverse: bool) -> f64 {
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
    ///Return disjoint list of edges that are in edges1 but not in edges2
    pub fn disjoint_edges(edges1: &Vec<Rc<RefCell<Edge>>>, edges2: Vec<Rc<RefCell<Edge>>>) -> Vec<Rc<RefCell<Edge>>> {
        edges1.uniq(edges2)
    }

    #[inline]
    ///union of 2 list of edges without duplicates
    pub fn union_edges(edges1: &Vec<Rc<RefCell<Edge>>>, edges2: Vec<Rc<RefCell<Edge>>>) -> Vec<Rc<RefCell<Edge>>> {
        edges1.union(edges2)
    }

    #[inline]
    ///Return unique list of weights with weight bigger then lower-bound and smaller then or equal to upperbound
    pub fn unique_weight_list(edges: &[Rc<RefCell<Edge>>], lower_bound: f64, upper_bound: f64) -> Vec<f64> {
        let mut weights = Vec::new();
        edges.iter().for_each(|edge| {
            if edge.borrow().get_weight() > lower_bound && edge.borrow().get_weight() <= upper_bound {
                weights.push(edge.borrow().get_weight());
                //weights.contains(&edge.get_weight());
            }
        });
        weights.unique()
    }

    #[inline]
    ///Return unique list of weights with weight bigger then lower-bound and smaller then or equal to upperbound
    pub fn unique_weight_list_above_or_eq(edges: &[Rc<RefCell<Edge>>], threshold: f64) -> Vec<f64> {
        let mut weights = Vec::new();
        edges.iter().for_each(|edge| {
            if edge.borrow().get_weight() >= threshold {
                weights.push(edge.borrow().get_weight());
            }
        });
        weights.unique()
    }

    #[inline]
    ///Return vector slice with weight bigger then lower-bound en smaller then or equal to upper bound
    pub fn relevant_slice(weights: &[f64], lower_bound: f64, upper_bound: f64) -> Vec<f64> {
        weights.iter().filter(|&&x| x > lower_bound && x <= upper_bound).cloned().collect()
    }

    #[inline]
    //sort list of weights and return median (O(n log n)), use quick_select for faster performance
    pub fn median(uniq_weights: &mut Vec<f64>) -> f64 {
        uniq_weights.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let middle = uniq_weights.len() / 2;
        if uniq_weights.len() % 2 == 0 {
            if uniq_weights[middle] < uniq_weights[middle - 1] {
                uniq_weights[middle]
            } else {
                uniq_weights[middle - 1]
            }
        } else {
            uniq_weights[middle]
        }
    }
}