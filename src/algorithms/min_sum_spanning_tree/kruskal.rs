use std::cell::RefCell;
use std::rc::Rc;
use log::{debug, error, info, trace};
use crate::algorithms::min_sum_spanning_tree::kruskal::ConnectionType::{Connected, Disconnected};
use crate::algorithms::util::Util;
use crate::datastructures::graph::edge::Edge;
use crate::datastructures::graph::mutable_graph::MutableGraph;
use crate::datastructures::uf::union_find::UF;
use crate::print_edges;

const FLOATING_POINT_EPSILON: f64 = 1.0E-12;

pub enum CalculationType {
    Cost,
    Weight,
}

pub enum ConnectionType {
    Connected,
    Disconnected,
}

pub struct Kruskal();

impl Kruskal {
    /// Returns a minimal spanning tree of the given graph, the total weight/cost of the tree and the bottleneck WEIGHT (not cost) of the tree.
    pub fn run(graph: &mut MutableGraph, calculation_type: CalculationType) -> (Option<MutableGraph>, f64, f64) {
        let start = std::time::Instant::now();
        Self::sort_by_calculation_type(graph, &calculation_type);
        let end = start.elapsed().as_nanos() as f64 / 1_000_000.0;
        trace!("Kruskal sorting over {} edges took {} ms", graph.edges().len(), end);
        Self::sorted_build(graph, calculation_type)
    }

    /// Returns a minimal spanning tree of the given graph, the total weight/cost of the tree and the bottleneck WEIGHT (not cost) of the tree.
    /// If the graph is not connected, the minimum spanning forest is returned.
    pub fn run_with_disconnected(graph: &mut MutableGraph, calculation_type: CalculationType) -> (ConnectionType, MutableGraph, f64, f64) {
        let start = std::time::Instant::now();
        Self::sort_by_calculation_type(graph, &calculation_type);
        let end = start.elapsed().as_nanos() as f64 / 1_000_000.0;
        trace!("Kruskal sorting over {} edges took {} ms", graph.edges().len(), end);
        Self::sorted_build_with_disconnected_components(graph, calculation_type)
    }

    fn sort_by_calculation_type(graph: &mut MutableGraph, calculation_type: &CalculationType) {
        match calculation_type {
            CalculationType::Cost => graph.edges_mut().sort_by(|a, b| a.borrow().get_cost().partial_cmp(&b.borrow().get_cost()).unwrap()),
            CalculationType::Weight => graph.edges_mut().sort_by(|a, b| a.borrow().get_weight().partial_cmp(&b.borrow().get_weight()).unwrap()),
        }
    }

    fn sorted_build(graph: &mut MutableGraph, calculation_type: CalculationType) -> (Option<MutableGraph>, f64, f64) {
        let (uf, st_edges, mut weight, mut bottleneck) = Self::construct(graph, calculation_type);
        if uf.count() > 1 {
            weight = f64::INFINITY;
            bottleneck = f64::INFINITY;
            return (None, weight, bottleneck);
        }
        let st = MutableGraph::new(graph.nodes_copy(), st_edges);
        debug_assert!(st.is_spanning_tree());
        (Some(st), weight, bottleneck)
    }

    fn sorted_build_with_disconnected_components(graph: &mut MutableGraph, calculation_type: CalculationType) -> (ConnectionType, MutableGraph, f64, f64) {
        let (uf, st_edges, mut weight, mut bottleneck) = Self::construct(graph, calculation_type);
        let st = MutableGraph::new(graph.nodes_copy(), st_edges);
        if uf.count() > 1 {
            trace!("Kruskal found {} disconnected components", uf.count());
            return (Disconnected, st, weight, bottleneck);
        }
        debug_assert!(st.is_spanning_tree());
        (Connected, st, weight, bottleneck)
    }

    fn construct(graph: &mut MutableGraph, calculation_type: CalculationType) -> (UF, Vec<Rc<RefCell<Edge>>>, f64, f64) {
        let mut st_edges = Vec::new();
        let mut weight = 0.0;
        let inverse = matches!(graph.edges()[0].borrow().get_weight(), w if w < 0.0);
        let mut bottleneck = match inverse {
            true => f64::NEG_INFINITY,
            false => f64::INFINITY,
        };
        let mut uf = UF::new(graph.nodes().len() as i32);
        for edge in graph.edges() {
            let (v, w) = edge.borrow().endpoints();
            if !uf.connected(v, w) {
                uf.union(v, w);
                st_edges.push(Rc::clone(edge));
                match calculation_type {
                    CalculationType::Cost => {
                        weight += edge.borrow().get_cost();
                        bottleneck = Util::update_bottleneck(bottleneck, edge, inverse);
                    }
                    CalculationType::Weight => {
                        weight += edge.borrow().get_weight();
                        bottleneck = Util::update_bottleneck(bottleneck, edge, inverse);
                    }
                }
            }
        }
        (uf, st_edges, weight, bottleneck)
    }

    /// Slow check if optimality conditions are met.
    fn check_optimality(st: &MutableGraph, weight: f64, calculation_type: CalculationType) -> bool {
        debug!("Checking optimality of MST");
        if st.edges().is_empty() {
            return true;
        }
        //check weight
        let mut total_weight = 0.0;
        for edge in st.edges() {
            match calculation_type {
                CalculationType::Cost => total_weight += edge.borrow().get_cost(),
                CalculationType::Weight => total_weight += edge.borrow().get_weight(),
            }
        }
        if (weight - total_weight).abs() > FLOATING_POINT_EPSILON {
            error!("Weight of edges does not equal weight: {} vs. {}", weight, total_weight);
            return false;
        }
        debug!("Weight of MST is valid");
        // check that it is acyclic
        let mut uf = UF::new(st.nodes().len() as i32);
        for edge in st.edges() {
            let (v, w) = edge.borrow().endpoints();
            if uf.connected(v, w) {
                error!("Not a forest");
                return false;
            }
            uf.union(v, w);
        }
        debug!("MST is acyclic");
        // check that it is a minimal spanning forest (cut optimality conditions)
        debug!("Checking cut optimality conditions");
        let mut i = 0;
        for edge in st.edges() {
            i += 1;
            if i % 100 == 0 {
                trace!("Progress: {}/{}", i, st.edges().len());
            }
            // all edges in MST except e
            uf = UF::new(st.nodes().len() as i32);
            for e in st.edges() {
                if Rc::ptr_eq(e, edge) {
                    continue;
                }
                let (v, w) = e.borrow().endpoints();
                uf.union(v, w);
            }
            // check that e is min weight edge in crossing cut
            for e in st.edges() {
                let (v, w) = e.borrow().endpoints();
                if !uf.connected(v, w) {
                    match calculation_type {
                        CalculationType::Cost => {
                            if e.borrow().get_cost() < edge.borrow().get_cost() {
                                error!("Edge {:?} violates cut optimality conditions", edge);
                                return false;
                            }
                        }
                        CalculationType::Weight => {
                            if e.borrow().get_weight() < edge.borrow().get_weight() {
                                error!("Edge {:?} violates cut optimality conditions", edge);
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use super::*;
    use crate::datastructures::graph::edge::Edge;
    use crate::datastructures::graph::node::Node;

    #[test]
    fn test_kruskal() {
        let mut nodes = Vec::new();
        for i in 0..8 {
            nodes.push(Rc::new(Node::default(i)));
        }
        let mut edges = Vec::new();
        vec![
            (0, 1, 1.0),
            (0, 2, 1.0),
            (0, 3, 1.0),
            (0, 4, 1.0),
            (0, 5, 1.0),
            (0, 6, 1.0),
            (0, 7, 1.0),
            (1, 2, 1.0),
            (1, 3, 1.0),
            (1, 4, 1.0),
            (1, 5, 1.0),
            (1, 6, 1.0),
            (1, 7, 1.0),
            (2, 3, 1.0),
            (2, 4, 1.0),
            (2, 5, 1.0),
            (2, 6, 1.0),
            (2, 7, 1.0),
            (3, 4, 1.0),
            (3, 5, 1.0),
            (3, 6, 1.0),
            (3, 7, 1.0),
            (4, 5, 1.0),
            (4, 6, 1.0),
            (4, 7, 1.0),
            (5, 6, 1.0),
            (5, 7, 1.0),
            (6, 7, 1.0),
        ].iter().for_each(|(v, w, weight)| {
            edges.push(Rc::new(RefCell::new(Edge::new(*v, *w).weight(*weight))));
        });
        let mut graph = MutableGraph::new(Rc::new(nodes), edges);
        let (st, weight, bottleneck) = Kruskal::run(&mut graph, CalculationType::Weight);
        assert!(st.is_some());
        assert!(st.unwrap().is_spanning_tree());
        assert_eq!(weight, 7.0);
        assert_eq!(bottleneck, 1.0);
    }

    #[test]
    fn test_kruskal2() {
        let mut nodes = Vec::new();
        for i in 0..=8 {
            nodes.push(Rc::new(Node::default(i)));
        }
        let mut edges_mut = Vec::new();
        vec![
            (7, 6, 1.0),
            (8, 2, 2.0),
            (6, 5, 2.0),
            (0, 1, 4.0),
            (2, 5, 4.0),
            (8, 6, 6.0),
            (2, 3, 7.0),
            (7, 8, 7.0),
            (0, 7, 8.0),
            (1, 2, 8.0),
            (3, 4, 9.0),
            (5, 4, 10.0),
            (1, 7, 11.0),
            (3, 5, 14.0),
        ].iter().for_each(|(v, w, weight)| {
            edges_mut.push(Rc::new(RefCell::new(Edge::new(*v, *w).weight(*weight))))
        });
        let nodes_rc = Rc::new(nodes);
        let mut graph_mut = MutableGraph::new(Rc::clone(&nodes_rc), edges_mut);
        let (st_mut, weight_mut, bottleneck_mut) = Kruskal::run(&mut graph_mut, CalculationType::Weight);
        assert!(st_mut.is_some());
        assert!(st_mut.unwrap().is_spanning_tree());
        assert_eq!(bottleneck_mut, 1.0);
        assert_eq!(weight_mut, 37.0);
    }

    #[test]
    fn test_kruskal3() {
        let mut nodes = Vec::new();
        for i in 0..=8 {
            nodes.push(Rc::new(Node::default(i)));
        }
        let mut edges = Vec::new();
        vec![
            (7, 6, 1.0),
            (8, 2, 2.0),
            (6, 5, 2.0),
            (0, 1, 4.0),
            (2, 5, 4.0),
            (8, 6, 6.0),
            (2, 3, 7.0),
            (7, 8, 7.0),
            (0, 7, 8.0),
            (1, 2, 8.0),
            (3, 4, 9.0),
            (5, 4, 10.0),
            (1, 7, 11.0),
            (3, 5, 14.0),
        ].iter().for_each(|(v, w, weight)| {
            edges.push(Rc::new(RefCell::new(Edge::new(*v, *w).cost(*weight))));
        });
        let mut graph = MutableGraph::new(Rc::new(nodes), edges);
        let (st, weight, bottleneck) = Kruskal::run(&mut graph, CalculationType::Cost);
        assert!(st.is_some());
        assert!(st.unwrap().is_spanning_tree());
        assert_eq!(weight, 37.0);
    }
}
