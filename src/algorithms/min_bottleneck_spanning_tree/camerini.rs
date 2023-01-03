use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;
use log::{debug, info};
use crate::algorithms::quick_select::QuickSelect;
use crate::datastructures::graph::edge::Edge;
use crate::datastructures::graph::immutable_graph::ImmutableGraph;
use crate::datastructures::graph::node::Node;
use crate::datastructures::uf::union_find::UF;

pub struct MBST();

impl MBST {
    /// Returns the minimum bottleneck spanning tree and the bottleneck of the given graph using the algorithm of Camerini et al (linear time).
    pub fn run(graph: &mut ImmutableGraph) -> (Option<ImmutableGraph>, f64) {
        info!("Calculating MBST...");
        let st_edges = Self::recursive_search(graph);
        let (corrected_st_edges, bottleneck) = Self::correct_st_edges(&st_edges);
        let st = ImmutableGraph::new(graph.nodes_copy(), corrected_st_edges);
        debug_assert!(st.is_spanning_tree());
        debug!("MBST cost: {}", st.calculate_total_cost());
        debug!("MBST bottleneck: {}", bottleneck);
        (Some(st), bottleneck)
    }

    fn recursive_search(graph: &mut ImmutableGraph) -> Vec<Rc<Edge>> {
        if graph.edges().len() == 1 {
            return graph.edges_copy();
        }
        let mut res = Vec::with_capacity(graph.nodes().len() - 1); // the spanning tree has n-1 edges
        let median = QuickSelect::find_median(graph.edges_mut());
        let mut big_half = Vec::new();
        let mut small_half = Vec::new();
        let mut uf = UF::new(graph.nodes().len() as i32);
        for edge in graph.edges_mut() {
            if Self::compare_edge(edge, &median) == Ordering::Greater {
                big_half.push(Rc::clone(edge));
            } else {
                small_half.push(Rc::clone(edge));
                let (u, v) = edge.endpoints();
                if !uf.connected(u, v) {
                    uf.union(u, v);
                    res.push(Rc::clone(edge));
                }
            }
        }
        if big_half.is_empty() {
            return res;
        }
        if uf.count() == 1 {
            let mut small_half_graph = ImmutableGraph::new(graph.nodes_copy(), small_half);
            return Self::recursive_search(&mut small_half_graph);
        }
        let mut super_graph = Self::build_super_graph(&big_half, &mut uf);
        res.append(&mut Self::recursive_search(&mut super_graph));
        res
    }

    fn build_super_graph(big_half: &Vec<Rc<Edge>>, uf: &mut UF) -> ImmutableGraph {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut map: HashMap<usize, usize> = HashMap::new();
        let mut ids = 0;
        for edge in big_half {
            let (u, v) = edge.endpoints();
            let u_parent = uf.find(u);
            let v_parent = uf.find(v);
            if let std::collections::hash_map::Entry::Vacant(e) = map.entry(u_parent) {
                e.insert(ids);
                nodes.push(Rc::new(Node::default(ids)));
                ids += 1;
            }
            if let std::collections::hash_map::Entry::Vacant(e) = map.entry(v_parent) {
                e.insert(ids);
                nodes.push(Rc::new(Node::default(ids)));
                ids += 1;
            }
            let (orig_u, orig_v) = edge.original_endpoints();
            let super_edge = Edge::new(map[&u_parent], map[&v_parent])
                .weight(edge.get_weight())
                .set_original_endpoints(orig_u, orig_v);
            edges.push(Rc::new(super_edge));
        }
        ImmutableGraph::new(nodes, edges)
    }

    fn correct_st_edges(st_edges: &[Rc<Edge>]) -> (Vec<Rc<Edge>>, f64) {
        let mut res = Vec::new();
        let mut bottleneck = f64::INFINITY;
        st_edges.iter().for_each(|edge| {
            let (u, v) = edge.endpoints();
            let (orig_u, orig_v) = edge.original_endpoints();
            if edge.get_weight() < bottleneck { bottleneck = edge.get_weight(); }
            if u == orig_u && v == orig_v {
                res.push(Rc::clone(edge));
            } else {
                let corrected_edge = Edge::new(orig_u, orig_v)
                    .weight(edge.get_weight());
                res.push(Rc::new(corrected_edge));
            }
        });
        (res, bottleneck)
    }

    fn compare_edge(edge1: &Rc<Edge>, edge2: &Rc<Edge>) -> Ordering {
        if edge1.get_weight() == edge2.get_weight() {
            let (v1, w1) = edge1.endpoints();
            let (v2, w2) = edge2.endpoints();
            if v1 == v2 {
                return w1.cmp(&w2);
            }
            return v1.cmp(&v2);
        }
        edge1.get_weight().total_cmp(&edge2.get_weight())
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::algorithms::min_bottleneck_spanning_tree::camerini::MBST;
    use crate::algorithms::min_sum_spanning_tree::kruskal::CalculationType;
    use crate::datastructures::graph::edge::Edge;
    use crate::datastructures::graph::immutable_graph::ImmutableGraph;
    use crate::datastructures::graph::node::Node;

    #[test]
    fn test_mbst() {
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
        let (_, _, bottleneck_kruskal) = graph.min_sum_st(CalculationType::Weight);
        let (st_cam, bottleneck_cam) = MBST::run(&mut graph);
        assert!(st_cam.is_some());
        assert!(st_cam.unwrap().is_spanning_tree());
        assert_eq!(bottleneck_cam, bottleneck_kruskal);
    }
}