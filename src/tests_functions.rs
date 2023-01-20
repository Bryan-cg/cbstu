#[cfg(test)]
mod tests {
    use std::fs;
    use rand::Rng;
    use crate::algorithms::constrained_bottleneck_spanning_tree::berman::Berman;
    use crate::algorithms::constrained_bottleneck_spanning_tree::edge_elimination::EdgeEliminationOld;
    use crate::algorithms::constrained_bottleneck_spanning_tree::fast_edge_elimination::FastEdgeElimination;
    use crate::algorithms::constrained_bottleneck_spanning_tree::punnen::Punnen;
    use crate::algorithms::util::Util;
    use crate::io::input_handler::InputHandler;
    use crate::print_edges;

    #[test]
    fn test_easy_instance() {
        let mut graph_mut = InputHandler::read_mut("test_data/abilene--D-B-M-N-C-A-N-N_network12_15.json");
        graph_mut.inverse_weights();
        let mut duplicated_graph = Util::duplicate_edges(&graph_mut);
        let (_, _, bottleneck_big_budget_berman) = Berman::run(&mut duplicated_graph, 10000.0);
        let (_, _, bottleneck_small_budget_berman) = Berman::run(&mut duplicated_graph, 100.0);
        let (_, _, bottleneck_big_budget_punnen) = Punnen::run(&mut duplicated_graph, 10000.0);
        let (_, _, bottleneck_small_budget_punnen) = Punnen::run(&mut duplicated_graph, 100.0);
        let (_, _, bottleneck_big_budget_edg, _) = EdgeEliminationOld::run(&mut duplicated_graph, 10000.0);
        let (_, _, bottleneck_small_budget_edg, _) = EdgeEliminationOld::run(&mut duplicated_graph, 100.0);
        let (_, _, bottleneck_big_budget_ee, _) = FastEdgeElimination::run(duplicated_graph, 10000.0);
        graph_mut = InputHandler::read_mut("test_data/abilene--D-B-M-N-C-A-N-N_network12_15.json");
        graph_mut.inverse_weights();
        duplicated_graph = Util::duplicate_edges(&graph_mut);
        let (st, _, bottleneck_small_budget_ee, _) = FastEdgeElimination::run(duplicated_graph, 100.0);
        assert_eq!(bottleneck_big_budget_berman, -26.0);
        assert_eq!(bottleneck_small_budget_berman, -14.0);
        assert_eq!(bottleneck_big_budget_edg, -26.0);
        assert_eq!(bottleneck_small_budget_edg, -14.0);
        assert_eq!(bottleneck_big_budget_punnen, -26.0);
        assert_eq!(bottleneck_small_budget_punnen, -14.0);
        assert_eq!(bottleneck_big_budget_ee, -26.0);
        assert_eq!(bottleneck_small_budget_ee, -14.0);
    }

    #[test]
    fn test_difficult_instance1() {
        let mut graph_mut = InputHandler::read_mut("test_data/ta2--D-B-E-N-C-A-N-N_network65_108.json");
        graph_mut.inverse_weights();
        let mut duplicated_graph = Util::duplicate_edges(&graph_mut);
        let (_, _, bottleneck) = Berman::run(&mut duplicated_graph, 300.0);
        let (_, _, bottleneck2) = Punnen::run(&mut duplicated_graph, 300.0);
        let (_, _, bottleneck3, _) = EdgeEliminationOld::run(&mut duplicated_graph, 300.0);
        let (_, _, bottleneck4, _) = FastEdgeElimination::run(duplicated_graph, 300.0);
        assert_eq!(bottleneck, -13.0);
        assert_eq!(bottleneck2, -13.0);
        assert_eq!(bottleneck3, -13.0);
        assert_eq!(bottleneck4, -13.0);
    }

    #[test]
    fn test_difficult_instance2() {
        let mut graph_mut = InputHandler::read_mut("data/wrp4-11_network123_233.json");
        graph_mut.inverse_weights();
        let mut duplicated_graph = Util::duplicate_edges(&graph_mut);
        let (_, _, bottleneck) = Berman::run(&mut duplicated_graph, 1000.0);
        let (_, _, bottleneck_2) = Punnen::run(&mut duplicated_graph, 1000.0);
        let (_, _, bottleneck_3, _) = EdgeEliminationOld::run(&mut duplicated_graph, 1000.0);
        duplicated_graph = Util::duplicate_edges(&graph_mut);
        let (_, _, bottleneck_small) = Berman::run(&mut duplicated_graph, 100.0);
        let (_, _, bottleneck_small2) = Punnen::run(&mut duplicated_graph, 100.0);
        let (_, _, bottleneck_small3, _) = EdgeEliminationOld::run(&mut duplicated_graph, 100.0);
        duplicated_graph = Util::duplicate_edges(&graph_mut);
        let (_, _, bottleneck_mid) = Berman::run(&mut duplicated_graph, 700.0);
        let (_, _, bottleneck_mid2) = Punnen::run(&mut duplicated_graph, 700.0);
        let (_, _, bottleneck_mid3, _) = EdgeEliminationOld::run(&mut duplicated_graph, 700.0);
        duplicated_graph = Util::duplicate_edges(&graph_mut);
        let (_, _, bottleneck_big) = Berman::run(&mut duplicated_graph, 10000.0);
        let (_, _, bottleneck_big2) = Punnen::run(&mut duplicated_graph, 10000.0);
        let (_, _, bottleneck_big3, _) = EdgeEliminationOld::run(&mut duplicated_graph, 10000.0);
        duplicated_graph = Util::duplicate_edges(&graph_mut);
        let (_, _, bottleneck_4, _) = FastEdgeElimination::run(duplicated_graph, 1000.0);
        graph_mut = InputHandler::read_mut("data/wrp4-11_network123_233.json");
        graph_mut.inverse_weights();
        duplicated_graph = Util::duplicate_edges(&graph_mut);
        let (_, _, bottleneck_small_4, _) = FastEdgeElimination::run(duplicated_graph, 100.0);
        graph_mut = InputHandler::read_mut("data/wrp4-11_network123_233.json");
        graph_mut.inverse_weights();
        duplicated_graph = Util::duplicate_edges(&graph_mut);
        let (_, _, bottleneck_mid_4, _) = FastEdgeElimination::run(duplicated_graph, 700.0);
        graph_mut = InputHandler::read_mut("data/wrp4-11_network123_233.json");
        graph_mut.inverse_weights();
        duplicated_graph = Util::duplicate_edges(&graph_mut);
        let (_, _, bottleneck_big_4, _) = FastEdgeElimination::run(duplicated_graph, 10000.0);
        assert_eq!(bottleneck_small, -2.0);
        assert_eq!(bottleneck_small2, -2.0);
        assert_eq!(bottleneck_small3, -2.0);
        assert_eq!(bottleneck_small_4, -2.0);
        assert_eq!(bottleneck_mid, -8.0);
        assert_eq!(bottleneck_mid2, -8.0);
        assert_eq!(bottleneck_mid3, -8.0);
        assert_eq!(bottleneck_mid_4, -8.0);
        assert_eq!(bottleneck_big, -9.0);
        assert_eq!(bottleneck_big2, -9.0);
        assert_eq!(bottleneck_big3, -9.0);
        assert_eq!(bottleneck_big_4, -9.0);
        assert_eq!(bottleneck, -9.0);
        assert_eq!(bottleneck_2, -9.0);
        assert_eq!(bottleneck_3, -9.0);
        assert_eq!(bottleneck_4, -9.0);
    }

    #[test]
    fn random_test_equal_results() {
        let mut graph_mut = InputHandler::read_mut("test_data/germany50--D-B-L-N-C-A-N-N_network50_88.json");
        graph_mut.inverse_weights();
        let mut duplicated_graph = Util::duplicate_edges(&graph_mut);
        let (_, _, bottleneck) = Berman::run(&mut duplicated_graph, 150.0);
        let (_, _, bottleneck2) = Punnen::run(&mut duplicated_graph, 150.0);
        let (_, _, bottleneck3, _) = EdgeEliminationOld::run(&mut duplicated_graph, 150.0);
        let (_, _, bottleneck4, _) = FastEdgeElimination::run(duplicated_graph, 150.0);
        assert_eq!(bottleneck, bottleneck2);
        assert_eq!(bottleneck, bottleneck3);
        assert_eq!(bottleneck, bottleneck4);
    }

    #[test]
    fn random_test_equal_results2() {
        let mut graph_mut = InputHandler::read_mut("test_data/pioro40--D-B-M-N-C-A-N-N_network40_89.json");
        graph_mut.inverse_weights();
        let mut duplicated_graph = Util::duplicate_edges(&graph_mut);
        let (_, _, bottleneck) = Berman::run(&mut duplicated_graph, 200.0);
        let (_, _, bottleneck2) = Punnen::run(&mut duplicated_graph, 200.0);
        let (_, _, bottleneck3, _) = EdgeEliminationOld::run(&mut duplicated_graph, 200.0);
        let (_, _, bottleneck4, _) = FastEdgeElimination::run(duplicated_graph, 200.0);
        assert_eq!(bottleneck, bottleneck2);
        assert_eq!(bottleneck, bottleneck3);
        assert_eq!(bottleneck, bottleneck4);
    }

    #[test]
    fn test_all_with_random_budget() {
        let paths = fs::read_dir("test_data").unwrap();
        for path in paths {
            let path = path.unwrap().path();
            let path = path.to_str().unwrap();
            if path.ends_with(".json") {
                let budget = rand::thread_rng().gen_range(100.0..1000.0);
                let mut graph_mut = InputHandler::read_mut(path);
                graph_mut.inverse_weights();
                let mut duplicated_graph = Util::duplicate_edges(&graph_mut);
                let (_, _, bottleneck) = Berman::run(&mut duplicated_graph, budget);
                let (_, _, bottleneck2) = Punnen::run(&mut duplicated_graph, budget);
                let (_, _, bottleneck3, _) = EdgeEliminationOld::run(&mut duplicated_graph, budget);
                let (_, _, bottleneck4, _) = FastEdgeElimination::run(duplicated_graph, budget);
                if bottleneck != bottleneck2 || bottleneck != bottleneck3 || bottleneck != bottleneck4 {
                    panic!("Bottlenecks are not equal for {}, bottleneck Berman {}, bottleneck Punnen {}, bottleneck edge_elm {}, bottleneck EE {}", path, bottleneck, bottleneck2, bottleneck3, bottleneck4);
                }
            }
        }
    }
}