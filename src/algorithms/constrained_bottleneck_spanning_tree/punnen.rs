use std::rc::Rc;
use log::{debug, info, trace, warn};
use crate::algorithms::min_sum_spanning_tree::kruskal::CalculationType;
use crate::algorithms::quick_select::QuickSelect;
use crate::algorithms::util::Util;
use crate::datastructures::graph::edge::Edge;
use crate::datastructures::graph::immutable_graph::ImmutableGraph;
use crate::print_edges;

pub struct Punnen();

impl Punnen {
    pub fn run(original_graph: &ImmutableGraph, budget: f64) -> (Option<ImmutableGraph>, f64, f64) {
        info!("Solving Constrained bottleneck spanning tree problem with Punnen's algorithm");
        let mut graph = Util::duplicate_edges(original_graph);
        let (op_bst, bottleneck_mbst) = graph.min_bot_st();
        let total_cost = graph.calculate_total_cost();
        if total_cost <= budget {
            info!("MBST is valid solution [bottleneck: {}, cost: {}]", bottleneck_mbst, total_cost);
            return (op_bst, total_cost, bottleneck_mbst)
        }
        info!("MBST is not valid solution [bottleneck: {}, cost: {}]", bottleneck_mbst, total_cost);
        let (op_min_cost_st, cost, bottleneck_min_cost) = graph.min_sum_st(CalculationType::Cost);
        if op_min_cost_st.is_none() || cost > budget {
            info!("No valid solution found");
            return (None, 0.0, 0.0)
        }
        let lower_bound = bottleneck_mbst;
        let upper_bound = bottleneck_min_cost;
        info!("Lower bound: {}, Upper bound: {}", lower_bound, upper_bound);
        let mut graph_lower_bound = graph.get_edges_weight_lower_or_eq_than(lower_bound);
        //shadow variables
        let (op_min_cost_st, cost, bottleneck_min_cost) = graph_lower_bound.min_sum_st(CalculationType::Cost);
        if op_min_cost_st.is_none() {
            info!("No valid solution found");
            return (None, 0.0, 0.0)
        }
        if cost <= budget {
            info!("MCST lower bound is valid solution [bottleneck: {}, cost: {}]", bottleneck_min_cost, cost);
            return (op_min_cost_st, cost, bottleneck_min_cost)
        }
        info!("MCST lower bound is not valid solution [cost: {}]", cost);
        let disjoint_graph = graph.get_edges_weight_bigger_than(lower_bound);
        let union_edges = Util::union_edges(disjoint_graph.edges(), op_min_cost_st.unwrap().edges_copy());
        info!("Recursive search for valid solution");
        Self::recursive_find(&graph, budget, lower_bound, upper_bound, union_edges)
    }

    fn recursive_find(graph: &ImmutableGraph, budget: f64, mut lower_bound: f64, mut upper_bound: f64, union_edges: Vec<Rc<Edge>>) -> (Option<ImmutableGraph>, f64, f64) {
        trace!("Recursive find [lower bound: {}, upper bound: {}]", lower_bound, upper_bound);
        let mut l = Util::unique_weight_list(&union_edges, lower_bound, upper_bound);
        let median_unique = QuickSelect::find_median_f64(&mut l);
        let graph_union = ImmutableGraph::new(graph.nodes_copy(), union_edges);
        let mut graph_below_w = graph_union.get_edges_weight_lower_or_eq_than(median_unique);
        let (op_min_cost_st, cost, bottleneck_min_cost) = graph_below_w.min_sum_st(CalculationType::Cost);
        if op_min_cost_st.is_none() {
            warn!("No valid solution found - disconnected graph");
            return (None, 0.0, 0.0)
        }
        if cost > budget {
            trace!("Found infeasible solution: cost: {}", cost);
            let disjoint_graph = graph_union.get_edges_weight_bigger_than(median_unique);
            let new_union_edges = Util::union_edges(disjoint_graph.edges(), graph_below_w.edges_copy());
            lower_bound = median_unique;
            return Self::recursive_find(graph, budget, lower_bound, upper_bound, new_union_edges)
        }
        trace!("Feasible solution [bottleneck: {}, cost: {}]", bottleneck_min_cost, cost);
        if l.len() == 1 || l.len() == 2 {
            return (op_min_cost_st, cost, bottleneck_min_cost)
        }
        upper_bound = median_unique;
        Self::recursive_find(graph, budget, lower_bound, upper_bound, graph_below_w.edges_copy())
    }
}