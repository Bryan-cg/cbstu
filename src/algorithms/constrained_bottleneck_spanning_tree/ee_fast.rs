use std::collections::HashSet;
use std::rc::Rc;
use log::{debug, info, trace, warn};
use crate::algorithms::quick_select::QuickSelect;
use crate::algorithms::min_sum_spanning_tree::kruskal::CalculationType;
use crate::algorithms::util::{PivotChecked, PivotResult, Util};
use crate::datastructures::garbage::Garbage;
use crate::datastructures::graph::mutable_graph::MutableGraph;
use crate::print_edges;

pub struct EE();

impl EE {
    pub fn run(original_graph: &mut MutableGraph, duplicated_graph: &mut MutableGraph, budget: f64) -> (Option<MutableGraph>, f64, f64) {
        let (_, lower_bound) = original_graph.min_bot_st();
        let (_, upper_bound) = duplicated_graph.min_bot_st();
        //println!("lower bound: {}, upper bound: {}", lower_bound, upper_bound);
        //eliminate edges with weight < lower_bound from duplicated_graph
        let mut before = duplicated_graph.edges().len();
        duplicated_graph.edges_mut().retain(|edge| {
            edge.borrow().get_weight() <= lower_bound
        });

        duplicated_graph.edges_mut().retain(|e| {
            if e.borrow().is_upgraded() {
                e.borrow().get_or_weight() >= upper_bound
            } else {
                true
            }
        });
        let mut after = duplicated_graph.edges().len();
        trace!("before: {}, after: {}, eliminated {}", before, after, before - after);
        let mut unique_weights = Util::unique_weight_list_above_or_eq(duplicated_graph.edges(), upper_bound);
        unique_weights.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Self::dual_bound_search(duplicated_graph, &unique_weights, budget)
    }

    pub fn run_test(graph: MutableGraph, budget: f64) -> (Option<MutableGraph>, f64, f64) {
        let mut unique_weights = Util::unique_weight_list(graph.edges(), f64::NEG_INFINITY, 0.0);
        unique_weights.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let upper_bound = unique_weights[unique_weights.len() - 1];
        let lower_bound = unique_weights[0] + 1.0;
        //Self::dbs(graph, &unique_weights, budget)
        //Self::recursion(graph, &unique_weights, budget, lower_bound, upper_bound)
        //Self::bisection_alternative(graph, &unique_weights, budget, lower_bound, upper_bound)
        unimplemented!("Not implemented yet")
    }

    pub fn run_test_bin(graph: MutableGraph, budget: f64) -> (Option<MutableGraph>, f64, f64, Garbage) {
        let mut unique_weights = Util::unique_weight_list(graph.edges(), f64::NEG_INFINITY, 0.0);
        unique_weights.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let upper_bound = unique_weights[unique_weights.len() - 1];
        let lower_bound = unique_weights[0] + 1.0;
        //Self::dbs(graph, &unique_weights, budget)
        //Self::recursion(graph, &unique_weights, budget, lower_bound, upper_bound)
        Self::bisection_alternative(graph, &unique_weights, budget, lower_bound, upper_bound)
    }

    pub fn bisection_alternative(mut graph: MutableGraph, unique_weights: &Vec<f64>, budget: f64, mut lower_bound: f64, mut upper_bound: f64) -> (Option<MutableGraph>, f64, f64, Garbage) {
        trace!("Bisection search");
        let mut max = unique_weights.len();
        let mut min = 0;
        let mut pivot;
        let mut final_st = None;
        let mut cost = 0.0;
        let mut bottleneck = 0.0;
        let mut bin = Garbage::new();
        while min <= max {
            pivot = (max + min) / 2;
            let mut graph_w = graph.get_edges_weight_lower_or_eq_than(unique_weights[pivot]);
            match Self::check_pivot_bisection(&mut graph_w, budget) {
                PivotChecked::Feasible(st) => {
                    debug!("Feasible pivot [bottleneck: {}, cost: {}]", st.2, st.1);
                    max = pivot - 1;
                    final_st = Some(st.0);
                    cost = st.1;
                    bottleneck = st.2;
                    upper_bound = bottleneck;
                    bin.add(Rc::new(graph));
                    graph = graph_w;
                }
                PivotChecked::BudgetExceeded(st) => {
                    debug!("Budget exceeded pivot");
                    let disjoint_graph = graph.get_edges_weight_bigger_than(unique_weights[pivot]);
                    let union_edges = Util::union_edges(disjoint_graph.edges(), st.edges_copy());
                    let nodes = graph.nodes_copy();
                    bin.add(Rc::new(graph));
                    graph = MutableGraph::new(nodes, union_edges);
                    lower_bound = unique_weights[pivot];
                    min = pivot + 1;
                },
                PivotChecked::Infeasible => {
                    debug!("Infeasible pivot");
                    min = pivot + 1;
                }
            }
        }
        (final_st, cost, bottleneck, bin)
    }

    fn check_pivot_bisection(graph: &mut MutableGraph, budget: f64) -> PivotChecked {
        let (op_mst, cost, bottleneck) = graph.min_sum_st(CalculationType::Cost);
        match op_mst {
            Some(st) => {
                match cost {
                    cost if cost <= budget => PivotChecked::Feasible((st, cost, bottleneck)),
                    _ => PivotChecked::BudgetExceeded(st)
                }
            }
            None => PivotChecked::Infeasible
        }
    }

    pub fn recursion(graph: &mut MutableGraph, unique_weights: &Vec<f64>, budget: f64, mut lower_bound: f64, mut upper_bound: f64) -> (Option<MutableGraph>, f64, f64) {
        trace!("Recursive find [lower bound: {}, upper bound: {}]", lower_bound, upper_bound);
        let mut l = Util::relevant_slice(unique_weights, lower_bound, upper_bound);
        trace!("relevant slice: {:?}", l);
        let median_unique = QuickSelect::find_median_f64(&mut l);
        trace!("median: {}", median_unique);
        let mut graph_below_w = graph.get_edges_weight_lower_or_eq_than(median_unique);
        let (op_min_cost_st, cost, bottleneck_min_cost) = graph_below_w.min_sum_st(CalculationType::Cost);
        if op_min_cost_st.is_none() {
            warn!("No valid solution found - disconnected graph");
            return (None, 0.0, 0.0);
        }
        if cost > budget {
            trace!("Found infeasible solution: cost: {}", cost);
            let disjoint_graph = graph.get_edges_weight_bigger_than(median_unique);
            let new_union_edges = Util::union_edges(disjoint_graph.edges(), op_min_cost_st.unwrap().edges_copy());
            lower_bound = median_unique;
            return Self::recursion(&mut MutableGraph::new(graph.nodes_copy(), new_union_edges), &unique_weights, budget, lower_bound, upper_bound);
        }
        trace!("Feasible solution [bottleneck: {}, cost: {}]", bottleneck_min_cost, cost);
        if l.len() == 1 || l.len() == 2 {
            return (op_min_cost_st, cost, bottleneck_min_cost);
        }
        upper_bound = median_unique;
        Self::recursion(&mut graph_below_w, &unique_weights, budget, lower_bound, upper_bound)
    }

    fn dbs(graph: &mut MutableGraph, unique_weights: &Vec<f64>, budget: f64) -> (Option<MutableGraph>, f64, f64) {
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
            iterations += 1;
            pivot_a = (max + min) / 2;
            pivot_b = max - 1;
            pivot_a_weight = unique_weights[pivot_a];
            pivot_b_weight = unique_weights[pivot_b];
            match Util::check_pivot_quick(graph, pivot_a_weight, budget) { //run with pivots code cleanup
                PivotResult::Feasible(st) => {
                    debug!("Feasible pivot_a [bottleneck: {}, cost: {}]", st.2, st.1);
                    final_st = Some(st.0);
                    cost = st.1;
                    bottleneck = st.2;
                    max = pivot_a;
                    if min < max {
                        graph.edges_mut().retain(|e| e.borrow().get_weight() <= bottleneck);
                    }
                }
                PivotResult::Infeasible => {
                    debug!("Infeasible pivot_a");
                    match Util::check_pivot_quick(graph, pivot_b_weight, budget) {
                        PivotResult::Feasible(st) => {
                            debug!("Feasible pivot_b [bottleneck: {}, cost: {}]", st.2, st.1);
                            final_st = Some(st.0);
                            cost = st.1;
                            bottleneck = st.2;
                            min = pivot_a + 1;
                            max = pivot_b;
                            if min < max {
                                graph.edges_mut().retain(|e| e.borrow().get_weight() <= bottleneck);
                            }
                        }
                        PivotResult::Infeasible => {
                            debug!("Infeasible pivot_b");
                            min = pivot_b + 1;
                        }
                    }
                }
            }
        }
        (final_st, cost, bottleneck)
    }

    fn dual_bound_search(duplicated_graph: &mut MutableGraph, unique_weights: &Vec<f64>, budget: f64) -> (Option<MutableGraph>, f64, f64) {
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
            match Util::check_pivot_quick(duplicated_graph, pivot_a_weight, budget) { //run with pivots code cleanup
                PivotResult::Feasible(st) => {
                    debug!("Feasible pivot_a [bottleneck: {}, cost: {}]", st.2, st.1);
                    final_st = Some(st.0);
                    cost = st.1;
                    bottleneck = st.2;
                    max = pivot_a;
                    if min < max {
                        duplicated_graph.edges_mut().retain(|e| e.borrow().get_weight() <= bottleneck);
                    }
                }
                PivotResult::Infeasible => {
                    debug!("Infeasible pivot_a");
                    match Util::check_pivot_quick(duplicated_graph, pivot_b_weight, budget) {
                        PivotResult::Feasible(st) => {
                            debug!("Feasible pivot_b [bottleneck: {}, cost: {}]", st.2, st.1);
                            final_st = Some(st.0);
                            cost = st.1;
                            bottleneck = st.2;
                            min = pivot_a + 1;
                            max = pivot_b;
                            if min < max {
                                duplicated_graph.edges_mut().retain(|e| e.borrow().get_weight() <= bottleneck);
                            }
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