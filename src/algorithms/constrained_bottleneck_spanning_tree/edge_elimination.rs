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

pub struct EdgeElimination();

impl EdgeElimination {
    pub fn run(graph: &mut MutableGraph, budget: f64) -> (Option<MutableGraph>, f64, f64, Garbage) {
        trace!("Solving Constrained bottleneck spanning tree problem with Edge Elimination algorithm");
        let (op_bst, _, bottleneck_mbst) = graph.min_sum_st(CalculationType::Weight);
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

    //todo if cost is increased, then Berman speed is increased, but not in the edge elimination algorithm, find out why
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
        let mut sort = true;
        let mut bin = Garbage::new();
        while min < max {
            trace!("Min: {}, Max: {}", min, max);
            iterations += 1;
            pivot_a = (max + min) / 2;
            pivot_b = max - 1;
            pivot_a_weight = relevant_edges[pivot_a];
            pivot_b_weight = relevant_edges[pivot_b];
            checked_a = Self::check_pivot_ee(graph, pivot_a_weight, budget, sort);
            match checked_a.0 {
                PivotResult::Feasible(st) => {
                    trace!("Found feasible solution pivot_a [bottleneck {}, cost {}]", st.2, st.1);
                    graph.edges_mut().retain(|e| e.borrow().get_weight() <= st.2);
                    final_st = Some(st);
                    //bin.add(Rc::new(graph));
                    //graph = checked_a.1.unwrap();
                    max = pivot_a;
                    //sort = false;
                }
                PivotResult::Infeasible => {
                    checked_b = Self::check_pivot_ee(graph, pivot_b_weight, budget, sort);
                    match checked_b.0 {
                        PivotResult::Feasible(st) => {
                            trace!("Found feasible solution pivot_b [bottleneck {}, cost {}]", st.2, st.1);
                            //bin.add(Rc::new(graph));
                            //graph = checked_b.1.unwrap();
                            graph.edges_mut().retain(|e| e.borrow().get_weight() <= st.2);
                            final_st = Some(st);
                            min = pivot_a + 1;
                            max = pivot_b;
                            //sort = false;
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
                (Some(st.0), st.1, st.2, bin)
            }
            None => {
                warn!("No feasible solution found");
                (None, 0.0, 0.0, bin)
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

    //also determine weight st to update lowerbound
    fn check_pivot_ee(graph: &MutableGraph, pivot_weight: f64, budget: f64, sort: bool) -> (PivotResult, Option<MutableGraph>) {
        let mut graph_below_pivot = graph.get_edges_weight_lower_or_eq_than(pivot_weight);
        let (mut op_mst, mut cost, mut bottleneck) = (None, 0.0, 0.0);
        match sort {
            true => (op_mst, cost, bottleneck) = graph_below_pivot.min_sum_st(CalculationType::Cost),
            false => (op_mst, cost, bottleneck) = graph_below_pivot.sorted_build(CalculationType::Cost)
        }
        match op_mst {
            Some(st) => {
                match cost {
                    cost if cost <= budget => (PivotResult::Feasible((st, cost, bottleneck)), Some(graph_below_pivot)),
                    _ => (PivotResult::Infeasible, None)
                }
            }
            None => (PivotResult::Infeasible, None)
        }
    }
}