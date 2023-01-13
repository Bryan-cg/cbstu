use std::rc::Rc;
use log::{debug, trace};
use crate::algorithms::min_sum_spanning_tree::kruskal::CalculationType;
use crate::algorithms::util::{PivotResult, PivotResultMut, Util};
use crate::datastructures::graph::immutable_graph::ImmutableGraph;
use crate::datastructures::graph::mutable_graph::MutableGraph;

pub struct Berman();

impl Berman {
    pub fn run(original_graph: &MutableGraph, budget: f64) -> (Option<MutableGraph>, f64, f64) {
        trace!("Solving Constrained bottleneck spanning tree problem with Berman's algorithm");
        let mut graph = Util::duplicate_edges_mut(original_graph);
        //graph.edges_mut().sort_by(|a, b| a.borrow().get_weight().partial_cmp(&b.borrow().get_weight()).unwrap()); //todo use unstable sort
        //graph.edges_mut().sort_unstable_by(|a, b| a.borrow().get_weight().partial_cmp(&b.borrow().get_weight()).unwrap());
        // get list with unique weights
        let mut unique_weights = Util::unique_weight_list(graph.edges(), f64::NEG_INFINITY,  0.0);
        unique_weights.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Self::dual_bound_search(&graph, &unique_weights, budget)
    }

    fn bisection_search(graph: &MutableGraph, unique_weights: &Vec<f64>, budget: f64) -> (Option<MutableGraph>, f64, f64) {
        trace!("Bisection search");
        let mut max = unique_weights.len();
        let mut min = 0;
        let mut pivot;
        let mut final_st = None;
        let mut cost = 0.0;
        let mut bottleneck = 0.0;
        while min <= max {
            pivot = (max + min) / 2;
            match Self::check_pivot(graph, budget, pivot, unique_weights) {
                PivotResultMut::Feasible(st) => {
                    debug!("Feasible pivot [bottleneck: {}, cost: {}]", st.2, st.1);
                    max = pivot - 1;
                    final_st = Some(st.0);
                    cost = st.1;
                    bottleneck = st.2;
                },
                PivotResultMut::Infeasible => {
                    debug!("Infeasible pivot");
                    min = pivot + 1;
                }
            }
        }
        (final_st, cost, bottleneck)
    }

    fn dual_bound_search(graph: &MutableGraph, unique_weights: &Vec<f64>, budget: f64) -> (Option<MutableGraph>, f64, f64) {
        trace!("Dual bound search");
        //let mut max = graph.edges().len() - 1;
        //let mut min = graph.nodes().len() - 1;
        let mut max = unique_weights.len();
        let mut min = 0;
        let mut pivot_a;
        let mut pivot_b;
        let mut final_st = None;
        let mut cost = 0.0;
        let mut bottleneck = 0.0;
        let mut iterations = 0;
        while min < max {
            iterations += 1;
            pivot_a = (max + min) / 2; //todo floor
            pivot_b = max - 1;
            match Self::check_pivot(graph, budget, pivot_a, unique_weights) {
                PivotResultMut::Feasible(st) => {
                    debug!("Feasible pivot_a [bottleneck: {}, cost: {}]", st.2, st.1);
                    final_st = Some(st.0);
                    cost = st.1;
                    bottleneck = st.2;
                    max = pivot_a;
                }
                PivotResultMut::Infeasible => {
                    debug!("Infeasible pivot_a");
                    match Self::check_pivot(graph, budget, pivot_b, unique_weights) {
                        PivotResultMut::Feasible(st) => {
                            debug!("Feasible pivot_b [bottleneck: {}, cost: {}]", st.2, st.1);
                            final_st = Some(st.0);
                            cost = st.1;
                            bottleneck = st.2;
                            min = pivot_a + 1;
                            max = pivot_b;
                        }
                        PivotResultMut::Infeasible => {
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

    fn check_pivot(graph: &MutableGraph, budget: f64, pivot: usize, unique_weights: &[f64]) -> PivotResultMut {
        let mut pivot_edges = Vec::with_capacity(graph.edges().len());
        //let pivot_weight = graph.edges()[pivot].borrow().get_weight();
        let pivot_weight = unique_weights[pivot];
        debug!("Pivot weight: {}", pivot_weight);
        for edge in graph.edges() {
            if edge.borrow().get_weight() <= pivot_weight {
                pivot_edges.push(Rc::clone(edge));
            }
        }
        let mut pivot_graph = MutableGraph::new(graph.nodes_copy(), pivot_edges);
        debug!("Pivot graph edges: {}", pivot_graph.edges().len());
        let (op_st, cost, bottleneck) = pivot_graph.min_sum_st(CalculationType::Cost);
        match op_st {
            Some(st) => {
                if cost <= budget {
                    PivotResultMut::Feasible((st, cost, bottleneck))
                } else {
                    debug!("Infeasible cost: {}", cost);
                    PivotResultMut::Infeasible
                }
            }
            None => PivotResultMut::Infeasible
        }
    }
}