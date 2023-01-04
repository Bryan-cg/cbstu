use std::rc::Rc;
use log::{info, warn};
use crate::algorithms::min_sum_spanning_tree::kruskal::CalculationType;
use crate::algorithms::util::{PivotResult, Util};
use crate::datastructures::graph::edge::Edge;
use crate::datastructures::graph::immutable_graph::ImmutableGraph;
use crate::print_edges;

pub struct EdgeElimination();

impl EdgeElimination {
    pub fn run(original_graph: &ImmutableGraph, budget: f64) -> (Option<ImmutableGraph>, f64, f64) {
        info!("Solving Constrained bottleneck spanning tree problem with Edge Elimination algorithm");
        let mut graph = Util::duplicate_edges(original_graph);
        let (op_bst, bottleneck_mbst) = graph.min_bot_st();
        let total_cost = graph.calculate_total_cost();
        if total_cost <= budget {
            info!("MBST is valid solution [bottleneck: {}, cost: {}]", bottleneck_mbst, total_cost);
            return (op_bst, total_cost, bottleneck_mbst);
        }
        info!("MBST is not valid solution [bottleneck: {}, cost: {}]", bottleneck_mbst, total_cost);
        let mut remaining_graph = Self::eliminate_upgraded_edges_above_bottleneck(original_graph, bottleneck_mbst);//todo change to graph
        println!("Remaining edges length: {}", remaining_graph.edges().len());
        let (op_min_cost_st, _, bottleneck_min_cost) = remaining_graph.min_sum_st(CalculationType::Cost);
        if op_min_cost_st.is_none() {
            warn!("No valid solution found");
            return (None, 0.0, 0.0);
        }
        let mut relevant_edges = Self::edges_above_or_eq_bottleneck(&remaining_graph, bottleneck_mbst);
        relevant_edges.sort_by(|a, b| a.get_weight().partial_cmp(&b.get_weight()).unwrap());
        Self::dual_bound_search(&graph, remaining_graph.edges_copy(), relevant_edges, bottleneck_min_cost, budget)
    }

    fn dual_bound_search(graph: &ImmutableGraph, mut remaining: Vec<Rc<Edge>>, relevant_edges: Vec<Rc<Edge>>, mut min_weight: f64, budget: f64) -> (Option<ImmutableGraph>, f64, f64) {
        info!("Dual bound search");
        let mut max = relevant_edges.len() - 1;
        let mut min = 0_usize;
        let mut pivot_a;
        let mut pivot_b;
        let mut pivot_a_weight;
        let mut pivot_b_weight;
        let mut final_st = None;
        while min < max {
            pivot_a = (max + min) / 2;
            pivot_b = max - 1;
            pivot_a_weight = relevant_edges[pivot_a].get_weight();
            pivot_b_weight = relevant_edges[pivot_b].get_weight();
            if pivot_a_weight > min_weight {
                min = pivot_a + 1;
                continue;
            }
            let checked_a = Self::check_pivot(graph, remaining.clone(), pivot_a_weight, budget);
            match checked_a {
                PivotResult::Feasible(st) => {
                    info!("Found feasible solution [bottleneck {}, cost {}]", st.2, st.1);
                    min_weight = st.2;
                    final_st = Some(st);
                    remaining.retain(|e| e.get_weight() <= min_weight);
                    max = pivot_a;
                }
                PivotResult::Infeasible => {
                    let checked_b = Self::check_pivot(graph, remaining.clone(), pivot_b_weight, budget);
                    match checked_b {
                        PivotResult::Feasible(st) => {
                            info!("Found feasible solution [bottleneck {}, cost {}]", st.2, st.1);
                            min_weight = st.2;
                            final_st = Some(st);
                            remaining.retain(|e| e.get_weight() <= min_weight);
                            min = pivot_a + 1;
                            max = pivot_b;
                        }
                        PivotResult::Infeasible => {
                            info!("Infeasible pivot_b");
                            min = pivot_b + 1;
                        }
                    }
                }
            }
        }
        match final_st {
            Some(st) => (Some(st.0), st.1, st.2),
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

    fn eliminate_upgraded_edges_above_bottleneck(graph: &ImmutableGraph, bottleneck: f64) -> ImmutableGraph {
        let mut remaining = Vec::new();
        graph.edges().iter().for_each(|e| {
            let (u, v) = e.endpoints();
            let unupgraded_edge = Edge::new(u,v).weight(e.get_weight());
            remaining.push(Rc::new(unupgraded_edge));
            //todo: is this right?
            if e.get_weight() > bottleneck {
                let upgraded_edge = Edge::new(u,v)
                    .weight(e.get_upgraded_weight())
                    .cost(e.get_cost())
                    .upgraded(true);
                remaining.push(Rc::new(upgraded_edge));
            }
        });
        ImmutableGraph::new(graph.nodes_copy(), remaining)
    }

    fn edges_above_or_eq_bottleneck(graph: &ImmutableGraph, bottleneck: f64) -> Vec<Rc<Edge>> {
        graph
            .edges()
            .iter()
            .filter(|e| e.get_weight() >= bottleneck)
            .map(Rc::clone)
            .collect()
    }
}