use std::cell::{Ref, RefCell};
use std::cmp::Ordering;
use std::rc::Rc;
use log::{trace};
use crate::algorithms::quick_select::QuickSelect;
use crate::algorithms::util::Util;
use crate::datastructures::graph::edge::Edge;
use crate::datastructures::graph::mutable_graph::MutableGraph;
use crate::datastructures::graph::node::Node;
use crate::datastructures::uf::union_find::UF;
use crate::print_edges;

pub struct MBST();

impl MBST {
    pub fn run(graph: &mut MutableGraph) -> (Option<MutableGraph>, f64) {
        let st_edges = Self::recursive_search(graph);
        let bottleneck = Self::find_bottleneck(&st_edges);
        let st = MutableGraph::new(graph.nodes_copy(), st_edges);
        debug_assert!(st.is_spanning_tree());
        (Some(st), bottleneck)
    }

    fn recursive_search(graph: &mut MutableGraph) -> Vec<Rc<RefCell<Edge>>> {
        if graph.edges().len() == 1 {
            return graph.edges_copy();
        }
        let mut res = Vec::with_capacity(graph.nodes().len() - 1);
        let median = QuickSelect::find_median_edges_mut(graph.edges_mut());
        let mut big_half = Vec::new();
        let mut small_half = Vec::new();
        let mut uf = UF::new(graph.nodes().len() as i32);
        for edge in graph.edges() {
            if Self::compare_edge_ref(&edge.borrow(), &median.borrow()) == Ordering::Greater {
                big_half.push(Rc::clone(edge));
            } else {
                small_half.push(Rc::clone(edge));
                let (u, v) = edge.borrow().original_endpoints();
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
            let mut small_half_graph = MutableGraph::new(graph.nodes_copy(), small_half);
            return Self::recursive_search(&mut small_half_graph);
        }
        let mut super_graph = Self::build_super_graph(&big_half, &mut uf, graph.nodes().len());
        res.append(&mut Self::recursive_search(&mut super_graph));
        res
    }

    fn build_super_graph(big_half: &[Rc<RefCell<Edge>>], uf: &mut UF, n: usize) -> MutableGraph {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut keys = vec![(false, 0); n];
        let mut ids = 0;
        for edge in big_half {
            let (u, v) = edge.borrow().original_endpoints();
            let u_parent = uf.find(u);
            let v_parent = uf.find(v);
            if !keys[u_parent].0 {
                keys[u_parent] = (true, ids);
                ids += 1;
                nodes.push(Rc::new(Node::default(u)));
            }
            if !keys[v_parent].0 {
                keys[v_parent] = (true, ids);
                ids += 1;
                nodes.push(Rc::new(Node::default(v)));
            }
            edge.borrow_mut().set_original_endpoints_self(keys[u_parent].1, keys[v_parent].1);
            edges.push(Rc::clone(edge));
        }
        MutableGraph::new(Rc::new(nodes), edges)
    }

    fn find_bottleneck(st_edges: &[Rc<RefCell<Edge>>]) -> f64 {
        let inverse = matches!(st_edges[0].borrow().get_weight(), w if w < 0.0);
        let mut bottleneck = match inverse {
            true => f64::NEG_INFINITY,
            false => f64::INFINITY,
        };
        st_edges.iter().for_each(|edge| {
            bottleneck = Util::update_bottleneck(bottleneck, edge, inverse);
        });
        bottleneck
    }

    fn compare_edge_ref(edge1: &Ref<Edge>, edge2: &Ref<Edge>) -> Ordering {
        if edge1.get_weight() == edge2.get_weight() {
            let (v1, w1) = edge1.original_endpoints();
            let (v2, w2) = edge2.original_endpoints();
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
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::algorithms::min_bottleneck_spanning_tree::camerini::MBST;
    use crate::algorithms::min_sum_spanning_tree::kruskal::CalculationType;
    use crate::datastructures::graph::edge::Edge;
    use crate::datastructures::graph::mutable_graph::MutableGraph;
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
            edges.push(Rc::new(RefCell::new(Edge::new(*v, *w).weight(*weight))));
        });
        let mut graph = MutableGraph::new(Rc::new(nodes), edges);
        let (_, _, bottleneck_kruskal) = graph.mst(CalculationType::Weight);
        let (st_cam, bottleneck_cam) = MBST::run(&mut graph);
        assert!(st_cam.is_some());
        assert!(st_cam.unwrap().is_spanning_tree());
        assert_eq!(bottleneck_cam, bottleneck_kruskal);
    }

    #[test]
    fn test_negative_weights() {
        let mut nodes = Vec::new();
        for i in 0..8 {
            nodes.push(Rc::new(Node::default(i)));
        }
        let mut edges = Vec::new();
        vec![
            (0, 1, -1.0),
            (0, 2, -2.0),
            (0, 3, -3.0),
            (0, 4, -4.0),
            (0, 5, -5.0),
            (0, 6, -6.0),
            (0, 7, -7.0),
            (1, 2, -1.0),
            (1, 3, -2.0),
            (1, 4, -3.0),
            (1, 5, -4.0),
            (1, 6, -5.0),
            (1, 7, -6.0),
            (2, 3, -1.0),
            (2, 4, -2.0),
            (2, 5, -3.0),
            (2, 6, -4.0),
            (2, 7, -5.0),
            (3, 4, -1.0),
            (3, 5, -2.0),
            (3, 6, -3.0),
            (3, 7, -4.0),
            (4, 5, -1.0),
            (4, 6, -2.0),
            (4, 7, -3.0),
            (5, 6, -1.0),
            (5, 7, -2.0),
            (6, 7, -1.0),
        ].iter().for_each(|(v, w, weight)| {
            edges.push(Rc::new(RefCell::new(Edge::new(*v, *w).weight(*weight))));
        });
        let mut graph = MutableGraph::new(Rc::new(nodes), edges);
        let (_, _, bottleneck_kruskal) = graph.mst(CalculationType::Weight);
        let (st_cam, bottleneck_cam) = MBST::run(&mut graph);
        assert!(st_cam.is_some());
        assert!(st_cam.unwrap().is_spanning_tree());
        assert_eq!(bottleneck_cam, bottleneck_kruskal);
        assert_eq!(bottleneck_cam, -4.0);
    }

    #[test]
    fn test_mutable_graph() {
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
            edges.push(Rc::new(RefCell::new(Edge::new(*v, *w).weight(*weight))));
        });
        let mut graph = MutableGraph::new(Rc::new(nodes), edges);
        let (st_cam, bottleneck_cam) = MBST::run(&mut graph);
        assert!(st_cam.is_some());
        assert_eq!(bottleneck_cam, 1.0);
    }
}