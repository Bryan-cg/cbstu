use std::rc::Rc;
use log::{debug, info, trace};
use crate::algorithms::min_sum_spanning_tree::kruskal::CalculationType;
use crate::algorithms::util::{PivotResult, Util};
use crate::datastructures::graph::mutable_graph::MutableGraph;

// Algorithm based on the paper "The Constrained Bottleneck Problem in Networks" by Berman et al.
pub struct Berman();

impl Berman {
    pub fn run(graph: &mut MutableGraph, budget: f64) -> (Option<MutableGraph>, f64, f64) {
        trace!("Solving Constrained bottleneck spanning tree problem with Berman's algorithm");
        let mut unique_weights = Util::unique_weight_list(graph.edges(), f64::NEG_INFINITY, 0.0);
        unique_weights.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Self::bisection_search(graph, &unique_weights, budget)
    }

    fn naive_search(graph: &mut MutableGraph, unique_weights: &Vec<f64>, budget: f64) -> (Option<MutableGraph>, f64, f64) {
        trace!("Naive search");
        let mut final_st = None;
        for pivot_weight in unique_weights {
            if let PivotResult::Feasible(st) = Util::check_pivot(graph, *pivot_weight, budget) {
                final_st = Some(st);
                break;
            }
        }
        match final_st {
            Some(st) => {
                trace!("Naive search finished [bottleneck {}, cost {}]", st.2, st.1);
                (Some(st.0), st.1, st.2)
            }
            None => {
                info!("No valid solution found");
                (None, 0.0, 0.0)
            }
        }
    }

    fn bisection_search(graph: &MutableGraph, unique_weights: &Vec<f64>, budget: f64) -> (Option<MutableGraph>, f64, f64) {
        trace!("Bisection search");
        let mut max = unique_weights.len() - 1;
        let mut min = 0;
        let mut pivot;
        let mut final_st = None;
        let mut cost = 0.0;
        let mut bottleneck = 0.0;
        while min <= max {
            pivot = ((max as f64 + min as f64) / 2.0).floor() as usize;
            match Util::check_pivot(graph, unique_weights[pivot], budget) {
                PivotResult::Feasible(st) => {
                    debug!("Feasible pivot [bottleneck: {}, cost: {}]", st.2, st.1);
                    max = pivot - 1;
                    final_st = Some(st.0);
                    cost = st.1;
                    bottleneck = st.2;
                }
                PivotResult::Infeasible => {
                    debug!("Infeasible pivot");
                    min = pivot + 1;
                }
            }
        }
        (final_st, cost, bottleneck)
    }

    // fn dual_bound_search(graph: &MutableGraph, unique_weights: &Vec<f64>, budget: f64) -> (Option<MutableGraph>, f64, f64) {
    //     trace!("Dual bound search");
    //     let mut max = unique_weights.len();
    //     let mut min = 0;
    //     let mut pivot_a;
    //     let mut pivot_b;
    //     let mut pivot_a_weight;
    //     let mut pivot_b_weight;
    //     let mut final_st = None;
    //     let mut cost = 0.0;
    //     let mut bottleneck = 0.0;
    //     let mut iterations = 0;
    //     while min < max {
    //         iterations += 1;
    //         pivot_a = (max + min) / 2;
    //         pivot_b = max - 1;
    //         pivot_a_weight = unique_weights[pivot_a];
    //         pivot_b_weight = unique_weights[pivot_b];
    //         match Util::check_pivot(graph, pivot_a_weight, budget) {
    //             PivotResultMut::Feasible(st) => {
    //                 debug!("Feasible pivot_a [bottleneck: {}, cost: {}]", st.2, st.1);
    //                 final_st = Some(st.0);
    //                 cost = st.1;
    //                 bottleneck = st.2;
    //                 max = pivot_a;
    //             }
    //             PivotResultMut::Infeasible => {
    //                 debug!("Infeasible pivot_a");
    //                 match Util::check_pivot(graph, pivot_b_weight, budget) {
    //                     PivotResultMut::Feasible(st) => {
    //                         debug!("Feasible pivot_b [bottleneck: {}, cost: {}]", st.2, st.1);
    //                         final_st = Some(st.0);
    //                         cost = st.1;
    //                         bottleneck = st.2;
    //                         min = pivot_a + 1;
    //                         max = pivot_b;
    //                     }
    //                     PivotResultMut::Infeasible => {
    //                         debug!("Infeasible pivot_b");
    //                         min = pivot_b + 1;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     trace!("Dual bound search finished [bottleneck: {}, cost: {}, iterations: {}]", bottleneck, cost, iterations);
    //     (final_st, cost, bottleneck)
    // }
}