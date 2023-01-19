use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use array_tool::vec::{Union, Uniq};
use fxhash::FxHashSet;
use log::{debug, trace};
use crate::algorithms::min_sum_spanning_tree::kruskal::CalculationType;
use crate::datastructures::graph::edge::Edge;
use crate::datastructures::graph::mutable_graph::MutableGraph;

///macro to print edges of Vec<Rc<Edge>>
#[macro_export]
macro_rules! print_edges {
    ($edges:expr) => {
        $edges.iter().for_each(|edge| {
            println!("{} - {}, {}, {}", edge.borrow().endpoints().0, edge.borrow().endpoints().1, edge.borrow().get_weight(), edge.borrow().is_upgraded());
        });
    };
}

pub enum PivotResult {
    Feasible((MutableGraph, f64, f64)),
    Infeasible,
}

pub enum PivotChecked{
    Feasible((MutableGraph, f64, f64)),
    BudgetExceeded(MutableGraph),
    Infeasible,
}

pub struct Util();

impl Util {
    #[inline]
    ///Creates a new graph with the same nodes, but each edge is duplicated with its original weight (cost 0) and upgraded weight (upgrade cost).
    pub fn duplicate_edges_mut(graph: &MutableGraph) -> MutableGraph {
        let mut edges = Vec::new();
        for edge in graph.edges() {
            let (u, v) = edge.borrow().endpoints();
            edges.push(Rc::new(RefCell::new(Edge::new(u, v).weight(edge.borrow().get_weight()).cost(0.0).upgraded(false))));
            edges.push(Rc::new(RefCell::new(Edge::new(u, v).weight(edge.borrow().get_upgraded_weight()).cost(edge.borrow().get_cost()).upgraded(true).or_weight(edge.borrow().get_weight()))));
        }
        MutableGraph::new(graph.nodes_copy(), edges)
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
    ///union of 2 list of edges without duplicates
    pub fn union_edges(edges1: &Vec<Rc<RefCell<Edge>>>, edges2: Vec<Rc<RefCell<Edge>>>) -> Vec<Rc<RefCell<Edge>>> {
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
        let mut weights = HashSet::new();
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
        weights.iter().filter(|&&x| x > lower_bound && x <= upper_bound).cloned().collect()//todo use retain
        //weights.retain(|&&x| x > lower_bound && x <= upper_bound);
        //weights
    }

    #[inline]
    pub(crate) fn check_pivot(graph: &MutableGraph, pivot_weight: f64, budget: f64) -> PivotResult {
        let mut graph_below_pivot = graph.get_edges_weight_lower_or_eq_than(pivot_weight);
        let (op_mst, cost, bottleneck) = graph_below_pivot.min_sum_st(CalculationType::Cost);
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

    #[inline]
    pub(crate) fn check_pivot_quick(graph: &MutableGraph, pivot_weight: f64, budget: f64) -> PivotResult {
        let mut graph_below_pivot = graph.get_edges_weight_lower_or_eq_than(pivot_weight);
        let (op_mst, cost, bottleneck) = graph_below_pivot.min_sum_early_detection(CalculationType::Cost, Some(budget));
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

    #[inline]
    pub fn dual_bound_search(graph: &mut MutableGraph, unique_weights: &Vec<f64>, budget: f64) -> (Option<MutableGraph>, f64, f64) {
        trace!("Dual bound search");
        let mut max = unique_weights.len();
        let mut min = 0;
        let mut pivot_a;
        let mut pivot_b;
        let mut pivot_a_weight;
        let mut pivot_b_weight;
        let mut final_st = None;
        let mut cost = 0.0;
        let mut bottleneck = 0.0;
        let mut iterations = 0;
        while min < max {
            trace!("Min: {}, Max: {}", min, max);
            iterations += 1;
            pivot_a = (max + min) / 2; //todo floor
            pivot_b = max - 1;
            pivot_a_weight = unique_weights[pivot_a];
            pivot_b_weight = unique_weights[pivot_b];
            match Util::check_pivot(graph, pivot_a_weight, budget) {
                PivotResult::Feasible(st) => {
                    debug!("Feasible pivot_a [bottleneck: {}, cost: {}]", st.2, st.1);
                    //graph.edges_mut().retain(|e| e.borrow().get_weight() <= st.2);
                    final_st = Some(st.0);
                    cost = st.1;
                    bottleneck = st.2;
                    max = pivot_a;
                }
                PivotResult::Infeasible => {
                    debug!("Infeasible pivot_a");
                    match Util::check_pivot(graph, pivot_b_weight, budget) {
                        PivotResult::Feasible(st) => {
                            debug!("Feasible pivot_b [bottleneck: {}, cost: {}]", st.2, st.1);
                            //graph.edges_mut().retain(|e| e.borrow().get_weight() <= st.2);
                            final_st = Some(st.0);
                            cost = st.1;
                            bottleneck = st.2;
                            min = pivot_a + 1;
                            max = pivot_b;
                        }
                        PivotResult::Infeasible => {
                            debug!("Infeasible pivot_b");
                            min = pivot_b + 1;
                        }
                    }
                }
            }
        }
        trace!("Dual bound search finished [bottleneck: {}, cost: {}, iterations: {}]", bottleneck, cost, iterations);
        (final_st, cost, bottleneck)
    }
}