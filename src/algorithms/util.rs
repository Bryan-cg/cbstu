use std::rc::Rc;
use crate::algorithms::min_sum_spanning_tree::kruskal::CalculationType;
use crate::datastructures::graph::edge::Edge;
use crate::datastructures::graph::immutable_graph::ImmutableGraph;

pub enum PivotResult {
    Feasible((ImmutableGraph, f64, f64)),
    Infeasible,
}

pub struct Util();

impl Util {
    ///Creates a new graph with the same nodes, but each edge is duplicated with its original weight (cost 0) and upgraded weight (upgrade cost).
    pub fn duplicate_edges(graph: &ImmutableGraph) -> ImmutableGraph {
        let mut edges = Vec::new();
        for edge in graph.edges() {
            let (u, v) = edge.endpoints();
            edges.push(Rc::new(Edge::new(u, v).weight(edge.get_weight()).cost(0.0).upgraded(false)));
            edges.push(Rc::new(Edge::new(u, v).weight(edge.get_upgraded_weight()).cost(edge.get_cost()).upgraded(true)));
        }
        ImmutableGraph::new(graph.nodes_copy(), edges)
    }

    pub fn update_bottleneck(bottleneck: f64, edge: &Rc<Edge>, inverse: bool) -> f64 {
        let mut bottleneck = bottleneck;
        if inverse {
            if edge.get_weight() > bottleneck {
                bottleneck = edge.get_weight();
            }
        } else if edge.get_weight() < bottleneck {
            bottleneck = edge.get_weight();
        }
        bottleneck
    }
}