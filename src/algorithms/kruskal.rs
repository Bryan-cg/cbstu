use std::rc::Rc;
use log::error;
use crate::datastructures::graph::immutable_graph::ImmutableGraph;
use crate::datastructures::uf::union_find::UF;

const FLOATING_POINT_EPSILON: f64 = 1.0E-12;

pub enum CalculationType {
    Cost,
    Weight,
}

pub struct Kruskal();

impl Kruskal {
    /// Returns a minimal spanning tree of the given graph, the total weight/cost of the tree and the bottleneck of the tree.
    pub fn run(graph: &mut ImmutableGraph, calculation_type: CalculationType) -> (Option<ImmutableGraph>, f64, f64) {
        let mut st_edges = Vec::new();
        let mut weight = 0.0;
        let mut bottleneck = f64::INFINITY;
        match calculation_type {
            CalculationType::Cost => graph.edges_mut().sort_by(|a, b| a.get_cost().partial_cmp(&b.get_cost()).unwrap()),
            CalculationType::Weight => graph.edges_mut().sort_by(|a, b| a.get_weight().partial_cmp(&b.get_weight()).unwrap()),
        }
        let mut uf = UF::new(graph.nodes().len() as i32);
        for edge in graph.edges() {
            let (v, w) = edge.endpoints();
            if !uf.connected(v, w) {
                uf.union(v, w);
                st_edges.push(Rc::clone(edge));
                match calculation_type {
                    CalculationType::Cost => {
                        weight += edge.get_cost();
                        if edge.get_cost() < bottleneck {
                            bottleneck = edge.get_cost();
                        }
                    }
                    CalculationType::Weight => {
                        weight += edge.get_weight();
                        if edge.get_weight() < bottleneck {
                            bottleneck = edge.get_weight();
                        }
                    }
                }
            }
        }
        if uf.count() > 1 {
            weight = f64::INFINITY;
            bottleneck = f64::INFINITY;
            return (None, weight, bottleneck);
        }
        let st = ImmutableGraph::new(graph.nodes_copy(), st_edges);
        debug_assert!(Self::check_optimality(&st, weight, calculation_type));
        (Some(st), weight, bottleneck)
    }

    fn check_optimality(st: &ImmutableGraph, weight: f64, calculation_type: CalculationType) -> bool {
        if st.edges().is_empty() {
            return true;
        }
        //check weight
        let mut total_weight = 0.0;
        for edge in st.edges() {
            match calculation_type {
                CalculationType::Cost => total_weight += edge.get_cost(),
                CalculationType::Weight => total_weight += edge.get_weight(),
            }
        }
        if (weight - total_weight).abs() > FLOATING_POINT_EPSILON {
            error!("Weight of edges does not equal weight: {} vs. {}", weight, total_weight);
            return false;
        }
        // check that it is acyclic
        let mut uf = UF::new(st.nodes().len() as i32);
        for edge in st.edges() {
            let (v, w) = edge.endpoints();
            if uf.connected(v, w) {
                error!("Not a forest");
                return false;
            }
            uf.union(v, w);
        }
        // check that it is a minimal spanning forest (cut optimality conditions)
        for edge in st.edges() {
            // all edges in MST except e
            uf = UF::new(st.nodes().len() as i32);
            for e in st.edges() {
                if Rc::ptr_eq(e, edge) {
                    continue;
                }
                let (v, w) = e.endpoints();
                uf.union(v, w);
            }
            // check that e is min weight edge in crossing cut
            for e in st.edges() {
                let (v, w) = e.endpoints();
                if !uf.connected(v, w) {
                    match calculation_type {
                        CalculationType::Cost => {
                            if e.get_cost() < edge.get_cost() {
                                error!("Edge {:?} violates cut optimality conditions", edge);
                                return false;
                            }
                        }
                        CalculationType::Weight => {
                            if e.get_weight() < edge.get_weight() {
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
    use super::*;
    use crate::datastructures::graph::immutable_graph::ImmutableGraph;
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
            edges.push(Rc::new(Edge::new(*v, *w).weight(*weight)));
        });
        let mut graph = ImmutableGraph::new(nodes, edges);
        let (st, weight, bottleneck) = Kruskal::run(&mut graph, CalculationType::Weight);
        assert!(st.is_some());
        assert!(st.unwrap().is_spanning_tree());
        assert_eq!(weight, 7.0);
        assert_eq!(bottleneck, 1.0);
    }

    #[test]
    fn test_kruskal2() {
        let mut nodes = Vec::new();
        for i in 0..8 {
            nodes.push(Rc::new(Node::default(i)));
        }
        let mut edges = Vec::new();
        vec![
            (0, 1, 1.0),
            (0, 2, 2.0),
            (0, 3, 3.0),
            (0, 4, 4.0),
            (0, 5, 5.0),
            (0, 6, 6.0),
            (0, 7, 7.0),
            (1, 2, 1.0),
            (1, 3, 2.0),
            (1, 4, 3.0),
            (1, 5, 4.0),
            (1, 6, 5.0),
            (1, 7, 6.0),
            (2, 3, 1.0),
            (2, 4, 2.0),
            (2, 5, 3.0),
            (2, 6, 4.0),
            (2, 7, 5.0),
            (3, 4, 1.0),
            (3, 5, 2.0),
            (3, 6, 3.0),
            (3, 7, 4.0),
            (4, 5, 1.0),
            (4, 6, 2.0),
            (4, 7, 3.0),
            (5, 6, 1.0),
            (5, 7, 2.0),
            (6, 7, 1.0),
        ].iter().for_each(|(v, w, weight)| {
            edges.push(Rc::new(Edge::new(*v, *w).weight(*weight)));
        });
        let mut graph = ImmutableGraph::new(nodes, edges);
        let (st, weight, bottleneck) = Kruskal::run(&mut graph, CalculationType::Weight);
        assert!(st.is_some());
        assert!(st.unwrap().is_spanning_tree());
        assert_eq!(weight, 7.0);
        assert_eq!(bottleneck, 1.0);
    }
}
