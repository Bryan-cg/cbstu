use std::cell::RefCell;
use std::rc::Rc;
use log::{trace, warn};
use array_tool::vec::{Uniq};
use crate::algorithms::min_sum_spanning_tree::kruskal::CalculationType;
use crate::algorithms::util::{PivotResult, PivotResultMut, Util};
use crate::datastructures::graph::edge::Edge;
use crate::datastructures::graph::immutable_graph::ImmutableGraph;
use crate::datastructures::graph::mutable_graph::MutableGraph;
use crate::print_edges;

pub struct EdgeElimination();

//todo check speedup, also use unique weights ?
impl EdgeElimination {
    pub fn run(original_graph: &MutableGraph, budget: f64) -> (Option<MutableGraph>, f64, f64) {
        trace!("Solving Constrained bottleneck spanning tree problem with Edge Elimination algorithm");
        let mut graph = Util::duplicate_edges_mut(original_graph);
        //let mut graph_fully_upgraded = Util::duplicate_only_upgraded(original_graph);
        // let (op_bst, bottleneck_mbst) = graph.min_bot_st();
        // let total_cost = graph.calculate_total_cost();
        let (op_bst, _, bottleneck_mbst) = graph.min_sum_st(CalculationType::Weight);
        let total_cost = graph.calculate_total_cost();
        if total_cost <= budget {
            trace!("MBST is valid solution [bottleneck: {}, cost: {}]", bottleneck_mbst, total_cost);
            return (op_bst, total_cost, bottleneck_mbst);
        }
        trace!("MBST is not valid solution [bottleneck: {}, cost: {}]", bottleneck_mbst, total_cost);
        let mut remaining_graph = Self::eliminate_upgraded_edges_above_bottleneck_mut(original_graph, bottleneck_mbst);
        let (op_min_cost_st, _, bottleneck_min_cost) = remaining_graph.min_sum_st(CalculationType::Cost);
        if op_min_cost_st.is_none() {
            warn!("No valid solution found");
            return (None, 0.0, 0.0);
        }
        let mut relevant_edges = Util::unique_weight_list_above_or_eq(remaining_graph.edges(), bottleneck_mbst); //25% speedup
        relevant_edges.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        Self::dual_bound_search(&mut remaining_graph, relevant_edges, bottleneck_min_cost, budget)
    }

    // fn bisection_search(graph: &MutableGraph, mut remaining: Vec<Rc<RefCell<Edge>>>, relevant_edges: Vec<Rc<RefCell<Edge>>>, mut min_weight: f64, budget: f64) -> (Option<MutableGraph>, f64, f64) {
    //     trace!("Bisection search");
    //     let mut max: i32 = relevant_edges.len() as i32;
    //     let mut min: i32 = 0;
    //     let mut pivot;
    //     let mut pivot_weight;
    //     let mut final_st = None;
    //     while min <= max {
    //         pivot = (max + min) / 2;
    //         pivot_weight = relevant_edges[pivot as usize].borrow().get_weight();
    //         match Self::check_pivot_mut(graph, remaining.clone(), pivot_weight, budget) {
    //             PivotResultMut::Feasible(st) => {
    //                 min_weight = st.2;
    //                 max = pivot - 1;
    //                 final_st = Some(st);
    //                 remaining.retain(|e| e.borrow().get_weight() <= min_weight);
    //             }
    //             PivotResultMut::Infeasible => {
    //                 min = pivot + 1;
    //             }
    //         }
    //     }
    //     match final_st {
    //         Some(st) => {
    //             trace!("Bisection search finished [bottleneck {}, cost {}]", st.2, st.1);
    //             (Some(st.0), st.1, st.2)
    //         }
    //         None => {
    //             warn!("No feasible solution found");
    //             (None, 0.0, 0.0)
    //         }
    //     }
    // }

    fn dual_bound_search(graph: &mut MutableGraph, relevant_edges: Vec<f64>, mut min_weight: f64, budget: f64) -> (Option<MutableGraph>, f64, f64) {
        trace!("Dual bound search");
        let mut max = relevant_edges.len();//todo test not -1
        let mut min = 0_usize;
        let mut pivot_a;
        let mut pivot_b;
        let mut pivot_a_weight;
        let mut pivot_b_weight;
        let mut final_st = None;
        let mut iterations = 0;
        while min < max {
            trace!("Min: {}, Max: {}", min, max);
            iterations += 1;
            pivot_a = (max + min) / 2;
            pivot_b = max - 1;
            pivot_a_weight = relevant_edges[pivot_a];
            pivot_b_weight = relevant_edges[pivot_b];
            // if pivot_a_weight > min_weight { //todo:check bug
            //     min = pivot_a + 1;
            //     continue;
            // }
            let checked_a = Self::check_pivot_mut(graph, pivot_a_weight, budget);
            match checked_a {
                PivotResultMut::Feasible(st) => {
                    trace!("Found feasible solution pivot_a [bottleneck {}, cost {}]", st.2, st.1);
                    min_weight = st.2;
                    final_st = Some(st);
                    graph.edges_mut().retain(|e| e.borrow().get_weight() <= min_weight);
                    max = pivot_a;
                }
                PivotResultMut::Infeasible => {
                    let checked_b = Self::check_pivot_mut(graph, pivot_b_weight, budget);
                    match checked_b {
                        PivotResultMut::Feasible(st) => {
                            trace!("Found feasible solution pivot_b [bottleneck {}, cost {}]", st.2, st.1);
                            min_weight = st.2;
                            final_st = Some(st);
                            graph.edges_mut().retain(|e| e.borrow().get_weight() <= min_weight);
                            min = pivot_a + 1;
                            max = pivot_b;
                        }
                        PivotResultMut::Infeasible => {
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
                (Some(st.0), st.1, st.2)
            }
            None => {
                warn!("No feasible solution found");
                (None, 0.0, 0.0)
            }
        }
    }

    fn check_pivot(graph: &ImmutableGraph, remaining: Vec<Rc<Edge>>, pivot_weight: f64, budget: f64) -> PivotResult {
        let remaining_graph = ImmutableGraph::new(graph.nodes_copy(), remaining);
        let mut graph_below_pivot = remaining_graph.get_edges_weight_lower_or_eq_than(pivot_weight);
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

    fn check_pivot_mut(graph: &MutableGraph, pivot_weight: f64, budget: f64) -> PivotResultMut {
        //let remaining_graph = MutableGraph::new(graph.nodes_copy(), remaining);
        let mut graph_below_pivot = graph.get_edges_weight_lower_or_eq_than(pivot_weight);
        let (op_mst, cost, bottleneck) = graph_below_pivot.min_sum_st(CalculationType::Cost);
        match op_mst {
            Some(st) => {
                match cost {
                    cost if cost <= budget => PivotResultMut::Feasible((st, cost, bottleneck)),
                    _ => PivotResultMut::Infeasible
                }
            }
            None => PivotResultMut::Infeasible
        }
    }


    fn eliminate_upgraded_edges_above_bottleneck(graph: &ImmutableGraph, bottleneck: f64) -> ImmutableGraph {
        let mut remaining = Vec::new();
        graph.edges().iter().for_each(|e| {
            let (u, v) = e.endpoints();
            let unupgraded_edge = Edge::new(u, v).weight(e.get_weight());
            remaining.push(Rc::new(unupgraded_edge));
            if e.get_weight() > bottleneck {
                let upgraded_edge = Edge::new(u, v)
                    .weight(e.get_upgraded_weight())
                    .cost(e.get_cost())
                    .upgraded(true);
                remaining.push(Rc::new(upgraded_edge));
            }
        });
        ImmutableGraph::new(graph.nodes_copy(), remaining)
    }

    fn eliminate_upgraded_edges_above_bottleneck_mut(graph: &MutableGraph, bottleneck: f64) -> MutableGraph {
        let mut remaining = Vec::new();
        graph.edges().iter().for_each(|e| {
            let (u, v) = e.borrow().endpoints();
            let unupgraded_edge = RefCell::new(Edge::new(u, v).weight(e.borrow().get_weight()));
            remaining.push(Rc::new(unupgraded_edge));
            if e.borrow().get_weight() > bottleneck {
                let upgraded_edge = Edge::new(u, v)
                    .weight(e.borrow().get_upgraded_weight())
                    .cost(e.borrow().get_cost())
                    .upgraded(true);
                remaining.push(Rc::new(RefCell::new(upgraded_edge)));
            }
        });
        MutableGraph::new(graph.nodes_copy(), remaining)
    }

    fn edges_above_or_eq_bottleneck(graph: &ImmutableGraph, bottleneck: f64) -> Vec<Rc<Edge>> {
        graph
            .edges()
            .iter()
            .filter(|e| e.get_weight() >= bottleneck)
            .map(Rc::clone)
            .collect()
    }

    fn edges_above_or_eq_bottleneck_mut(graph: &MutableGraph, bottleneck: f64) -> Vec<Rc<RefCell<Edge>>> {
        let edges: Vec<Rc<RefCell<Edge>>> = graph
            .edges()
            .iter()
            .filter(|e| e.borrow().get_weight() >= bottleneck)
            .map(Rc::clone)
            .collect();
        edges.unique()
    }
}