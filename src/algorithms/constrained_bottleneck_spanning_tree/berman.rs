use std::rc::Rc;
use log::{debug, info};
use crate::algorithms::min_sum_spanning_tree::kruskal::CalculationType;
use crate::algorithms::util::{PivotResult, Util};
use crate::datastructures::graph::immutable_graph::ImmutableGraph;

pub struct Berman();

impl Berman {
    pub fn run(original_graph: &ImmutableGraph, budget: f64) -> (Option<ImmutableGraph>, f64, f64) {
        info!("Solving Constrained bottleneck spanning tree problem with Berman's algorithm");
        let mut graph = Util::duplicate_edges(original_graph);
        graph.edges_mut().sort_by(|a, b| a.get_weight().partial_cmp(&b.get_weight()).unwrap());
        Self::dual_bound_search(&graph, budget)
    }

    fn dual_bound_search(graph: &ImmutableGraph, budget: f64) -> (Option<ImmutableGraph>, f64, f64) {
        info!("Dual bound search");
        let mut max = graph.edges().len() - 1;
        let mut min = graph.nodes().len() - 1;
        let mut pivot_a;
        let mut pivot_b;
        let mut final_st= None;
        let mut cost = 0.0;
        let mut bottleneck = 0.0;
        let mut iterations = 0;
        while min < max {
            iterations += 1;
            pivot_a = (max + min) / 2;
            pivot_b = max - 1;
            match Self::check_pivot(graph, budget, pivot_a) {
                PivotResult::Feasible(st) => {
                    debug!("Feasible pivot_a [bottleneck: {}, cost: {}]", st.2, st.1);
                    final_st = Some(st.0);
                    cost = st.1;
                    bottleneck = st.2;
                    max = pivot_a;
                }
                PivotResult::Infeasible => {
                    debug!("Infeasible pivot_a");
                    match Self::check_pivot(graph, budget, pivot_b) {
                        PivotResult::Feasible(st) => {
                            debug!("Feasible pivot_b [bottleneck: {}, cost: {}]", st.2, st.1);
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
        info!("Dual bound search finished [bottleneck: {}, cost: {}, iterations: {}]", bottleneck, cost, iterations);
        (final_st, cost, bottleneck)
    }

    fn check_pivot(graph: &ImmutableGraph, budget: f64, pivot: usize) -> PivotResult {
        let mut pivot_edges = Vec::with_capacity(graph.edges().len());
        let pivot_weight = graph.edges()[pivot].get_weight();
        debug!("Pivot weight: {}", pivot_weight);
        for edge in graph.edges() {
            if edge.get_weight() <= pivot_weight {
                pivot_edges.push(Rc::clone(edge));
            }
        }
        let mut pivot_graph = ImmutableGraph::new(graph.nodes_copy(), pivot_edges);
        debug!("Pivot graph edges: {}", pivot_graph.edges().len());
        let (op_st, cost, bottleneck) = pivot_graph.min_sum_st(CalculationType::Cost);
        match op_st {
            Some(st) => {
                if cost <= budget {
                    PivotResult::Feasible((st, cost, bottleneck))
                } else {
                    debug!("Infeasible cost: {}", cost);
                    PivotResult::Infeasible
                }
            }
            None => PivotResult::Infeasible
        }
    }
}