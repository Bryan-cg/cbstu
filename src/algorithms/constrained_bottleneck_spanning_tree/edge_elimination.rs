use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use log::{trace, warn};
use crate::algorithms::min_sum_spanning_tree::kruskal::CalculationType;
use crate::algorithms::util;
use crate::algorithms::util::{PivotResult, Util};
use crate::datastructures::garbage::Garbage;
use crate::datastructures::graph::edge::Edge;
use crate::datastructures::graph::mutable_graph::MutableGraph;
use crate::print_edges;

pub struct EdgeEliminationOld();

impl EdgeEliminationOld {
    pub fn run(graph: &mut MutableGraph, budget: f64) -> (Option<MutableGraph>, f64, f64, Garbage) {
        trace!("Solving Constrained bottleneck spanning tree problem with Edge Elimination algorithm");
        let (op_bst, _, bottleneck_mbst) = graph.mst(CalculationType::Weight);
        let total_cost = graph.calculate_total_cost();
        if total_cost <= budget {
            trace!("MBST is valid solution [bottleneck: {}, cost: {}]", bottleneck_mbst, total_cost);
            return (op_bst, total_cost, bottleneck_mbst, Garbage::default());
        }
        trace!("MBST is not valid solution [bottleneck: {}, cost: {}]", bottleneck_mbst, total_cost);
        Self::eliminate_upgraded_edges_above_bottleneck(graph, bottleneck_mbst);
        let mut relevant_edges = Util::unique_weight_list_above_or_eq(graph.edges(), bottleneck_mbst);
        relevant_edges.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Self::dual_bound_search(graph, relevant_edges, budget)
    }

    fn dual_bound_search(graph: &mut MutableGraph, relevant_edges: Vec<f64>, budget: f64) -> (Option<MutableGraph>, f64, f64, Garbage) {
        trace!("Dual bound search");
        let mut max = relevant_edges.len();
        let mut min = 0_usize;
        let mut pivot_a;
        let mut pivot_b;
        let mut pivot_a_weight;
        let mut pivot_b_weight;
        let mut final_st = None;
        let mut iterations = 0;
        let mut checked_a;
        let mut checked_b;
        while min < max {
            trace!("Min: {}, Max: {}", min, max);
            iterations += 1;
            pivot_a = (max + min) / 2;
            pivot_b = max - 1;
            pivot_a_weight = relevant_edges[pivot_a];
            pivot_b_weight = relevant_edges[pivot_b];
            checked_a = Util::check_pivot(graph, pivot_a_weight, budget);
            match checked_a {
                PivotResult::Feasible(st) => {
                    trace!("Found feasible solution pivot_a [bottleneck {}, cost {}]", st.2, st.1);
                    graph.edges_mut().retain(|e| e.borrow().get_weight() <= st.2);
                    final_st = Some(st);
                    max = pivot_a;
                }
                PivotResult::Infeasible => {
                    checked_b = Util::check_pivot(graph, pivot_b_weight, budget);
                    match checked_b {
                        PivotResult::Feasible(st) => {
                            trace!("Found feasible solution pivot_b [bottleneck {}, cost {}]", st.2, st.1);
                            graph.edges_mut().retain(|e| e.borrow().get_weight() <= st.2);
                            final_st = Some(st);
                            min = pivot_a + 1;
                            max = pivot_b;
                        }
                        PivotResult::Infeasible => {
                            trace!("Infeasible pivot_b");
                            min = pivot_b + 1;
                        }
                    }
                }
            }
        }
        match final_st {
            Some(st) => {
                trace!("Dual bound search finished [bottleneck {}, cost {}, iterations {}]", st.2, st.1, iterations);
                (Some(st.0), st.1, st.2, Garbage::default())
            }
            None => {
                warn!("No feasible solution found");
                (None, 0.0, 0.0, Garbage::default())
            }
        }
    }

    fn eliminate_upgraded_edges_above_bottleneck(graph: &mut MutableGraph, bottleneck: f64) {
        graph.edges_mut().retain(|e| {
            if e.borrow().is_upgraded() {
                e.borrow().get_or_weight() >= bottleneck
            } else {
                true
            }
        });
    }

}