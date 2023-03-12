use std::collections::HashSet;
use std::rc::Rc;
use log::{debug, info, trace, warn};
use crate::algorithms::quick_select::QuickSelect;
use crate::algorithms::min_sum_spanning_tree::kruskal::{CalculationType, ConnectionType};
use crate::algorithms::util::{Util};
use crate::datastructures::garbage::Garbage;
use crate::datastructures::graph::mutable_graph::MutableGraph;
use crate::print_edges;

enum PivotChecked {
    Feasible((MutableGraph, f64, f64)),
    Infeasible(MutableGraph),
}

// Calculating lower and upperbound beforehand is the bottleneck in the Punnen algorithm, so we avoid
// calculating bounds beforehand and use a binary search combined with updating the working graph
// (eliminating edges) to increase performance, a bin is returned with graphs that were created
// during the algorithm. This is necessary to avoid constantly dropping big graphs
// during the bisection search.
pub struct FastEdgeElimination();

impl FastEdgeElimination {
    pub fn run(graph: MutableGraph, budget: f64) -> (Option<MutableGraph>, f64, f64, Garbage) {
        let mut unique_weights = Util::unique_weight_list(graph.edges(), f64::NEG_INFINITY, 0.0);
        unique_weights.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Self::bisection_elimination_search(graph, &unique_weights, budget)
    }

    pub fn bisection_elimination_search(mut graph: MutableGraph, unique_weights: &Vec<f64>, budget: f64) -> (Option<MutableGraph>, f64, f64, Garbage) {
        trace!("Bisection search");
        let mut max = unique_weights.len() - 1;
        let mut min = 0;
        let mut pivot;
        let mut final_st = None;
        let mut cost = 0.0;
        let mut bottleneck = 0.0;
        let mut bin = Garbage::new();
        while min <= max {
            pivot = ((max as f64 + min as f64) / 2.0).floor() as usize;
            let mut graph_w = graph.smaller_or_eq_than(unique_weights[pivot]);
            match Self::check_pivot_bisection(&mut graph_w, budget) {
                PivotChecked::Feasible(st) => {
                    debug!("Feasible pivot [bottleneck: {}, cost: {}]", st.2, st.1);
                    max = pivot - 1;
                    final_st = Some(st.0);
                    cost = st.1;
                    bottleneck = st.2;
                    bin.add(Rc::new(graph));
                    graph = graph_w;
                }
                PivotChecked::Infeasible(st) => {
                    debug!("Budget exceeded pivot or disconnected spanning tree");
                    let edges_before = graph.edges().len();
                    let disjoint_graph = graph.bigger_than(unique_weights[pivot]);
                    let union_edges = Util::union_edges(disjoint_graph.edges(), st.edges());
                    let edges_after = union_edges.len();
                    trace!("Edges removed: {}", edges_before - edges_after);
                    let nodes = graph.nodes_copy();
                    bin.add(Rc::new(graph));
                    graph = MutableGraph::new(nodes, union_edges);
                    min = pivot + 1;
                }
            }
        }
        (final_st, cost, bottleneck, bin)
    }

    fn check_pivot_bisection(graph: &mut MutableGraph, budget: f64) -> PivotChecked {
        let (connection_type, st, cost, bottleneck) = graph.mst_disconnected(CalculationType::Cost);
        match connection_type {
            ConnectionType::Connected => {
                match cost {
                    cost if cost <= budget => PivotChecked::Feasible((st, cost, bottleneck)),
                    _ => PivotChecked::Infeasible(st)
                }
            }
            _ => PivotChecked::Infeasible(st)
        }
    }
}