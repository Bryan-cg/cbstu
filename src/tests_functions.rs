#[cfg(test)]
mod tests {
    use std::fs;
    use rand::Rng;
    use crate::algorithms::constrained_bottleneck_spanning_tree::berman::Berman;
    use crate::algorithms::constrained_bottleneck_spanning_tree::edge_elimination::EdgeElimination;
    use crate::algorithms::constrained_bottleneck_spanning_tree::punnen::Punnen;
    use crate::io::input_handler::InputHandler;

    #[test]
    fn test_easy_instance() {
        let graph = InputHandler::read("test_data/abilene--D-B-M-N-C-A-N-N_network12_15.json");
        let graph_mut = InputHandler::read_mut("test_data/abilene--D-B-M-N-C-A-N-N_network12_15.json");
        let neg_graph = graph.negative_weights();
        let (_, _, bottleneck_big_budget_berman) = Berman::run(&graph_mut, 10000.0);
        let (_, _, bottleneck_small_budget_berman) = Berman::run(&graph_mut, 100.0);
        let (_, _, bottleneck_big_budget_punnen) = Punnen::run(&graph_mut, 10000.0);
        let (_, _, bottleneck_small_budget_punnen) = Punnen::run(&graph_mut, 100.0);
        let (_, _, bottleneck_big_budget_edg) = EdgeElimination::run(&graph_mut, 10000.0);
        let (_, _, bottleneck_small_budget_edg) = EdgeElimination::run(&graph_mut, 100.0);
        assert_eq!(bottleneck_big_budget_berman, -26.0);
        assert_eq!(bottleneck_small_budget_berman, -14.0);
        assert_eq!(bottleneck_big_budget_edg, -26.0);
        assert_eq!(bottleneck_small_budget_edg, -14.0);
        assert_eq!(bottleneck_big_budget_punnen, -26.0);
        assert_eq!(bottleneck_small_budget_punnen, -14.0);
    }

    #[test]
    fn test_difficult_instance1() {
        let graph = InputHandler::read("test_data/ta2--D-B-E-N-C-A-N-N_network65_108.json");
        let neg_graph = graph.negative_weights();
        let graph_mut = InputHandler::read_mut("test_data/ta2--D-B-E-N-C-A-N-N_network65_108.json");
        let (_, _, bottleneck) = Berman::run(&graph_mut, 300.0);
        let (_, _, bottleneck2) = Punnen::run(&graph_mut, 300.0);
        let (_, _, bottleneck3) = EdgeElimination::run(&graph_mut, 300.0);
        assert_eq!(bottleneck, -13.0);
        assert_eq!(bottleneck2, -13.0);
        assert_eq!(bottleneck3, -13.0);
    }

    #[test]
    fn test_difficult_instance2() {
        let graph = InputHandler::read("data/wrp4-11_network123_233.json");
        let graph_mut = InputHandler::read_mut("data/wrp4-11_network123_233.json");
        let neg_graph = graph.negative_weights();
        let (_, _, bottleneck) = Berman::run(&graph_mut, 1000.0);
        let (_, _, bottleneck_2) = Punnen::run(&graph_mut, 1000.0);
        let (_, _, bottleneck_3) = EdgeElimination::run(&graph_mut, 1000.0);
        let (_, _, bottleneck_small) = Berman::run(&graph_mut, 100.0);
        let (_, _, bottleneck_small2) = Punnen::run(&graph_mut, 100.0);
        let (_, _, bottleneck_small3) = EdgeElimination::run(&graph_mut, 100.0);
        let (_, _, bottleneck_mid) = Berman::run(&graph_mut, 700.0);
        let (_, _, bottleneck_mid2) = Punnen::run(&graph_mut, 700.0);
        let (_, _, bottleneck_mid3) = EdgeElimination::run(&graph_mut, 700.0);
        let (_, _, bottleneck_big) = Berman::run(&graph_mut, 10000.0);
        let (_, _, bottleneck_big2) = Punnen::run(&graph_mut, 10000.0);
        let (_, _, bottleneck_big3) = EdgeElimination::run(&graph_mut, 10000.0);
        assert_eq!(bottleneck_small, -2.0);
        assert_eq!(bottleneck_small2, -2.0);
        assert_eq!(bottleneck_small3, -2.0);
        assert_eq!(bottleneck_mid, -8.0);
        assert_eq!(bottleneck_mid2, -8.0);
        assert_eq!(bottleneck_mid3, -8.0);
        assert_eq!(bottleneck_big, -9.0);
        assert_eq!(bottleneck_big2, -9.0);
        assert_eq!(bottleneck_big3, -9.0);
        assert_eq!(bottleneck, -9.0);
        assert_eq!(bottleneck_2, -9.0);
        assert_eq!(bottleneck_3, -9.0);
    }

    #[test]
    fn random_test_equal_results() {
        let graph = InputHandler::read("test_data/germany50--D-B-L-N-C-A-N-N_network50_88.json");
        let graph_mut = InputHandler::read_mut("test_data/germany50--D-B-L-N-C-A-N-N_network50_88.json");
        let neg_graph = graph.negative_weights();
        let (_, _, bottleneck) = Berman::run(&graph_mut, 150.0);
        let (_, _, bottleneck2) = Punnen::run(&graph_mut, 150.0);
        let (_, _, bottleneck3) = EdgeElimination::run(&graph_mut, 150.0);
        assert_eq!(bottleneck, bottleneck2);
        assert_eq!(bottleneck, bottleneck3);
    }

    #[test]
    fn random_test_equal_results2() {
        let graph = InputHandler::read("test_data/pioro40--D-B-M-N-C-A-N-N_network40_89.json");
        let graph_mut = InputHandler::read_mut("test_data/pioro40--D-B-M-N-C-A-N-N_network40_89.json");
        let neg_graph = graph.negative_weights();
        let (_, _, bottleneck) = Berman::run(&graph_mut, 200.0);
        let (_, _, bottleneck2) = Punnen::run(&graph_mut, 200.0);
        let (_, _, bottleneck3) = EdgeElimination::run(&graph_mut, 200.0);
        assert_eq!(bottleneck, bottleneck2);
        assert_eq!(bottleneck, bottleneck3);
    }

    #[test]
    fn test_all_with_random_budget() {
        let paths = fs::read_dir("test_data").unwrap();
        for path in paths {
            let path = path.unwrap().path();
            let path = path.to_str().unwrap();
            if path.ends_with(".json") {
                let budget = rand::thread_rng().gen_range(100.0..1000.0);
                let graph = InputHandler::read(path);
                let graph_mut = InputHandler::read_mut(path);
                let neg_graph = graph.negative_weights();
                let (_, _, bottleneck) = Berman::run(&graph_mut, budget);
                let (_, _, bottleneck2) = Punnen::run(&graph_mut, budget);
                let (_, _, bottleneck3) = EdgeElimination::run(&graph_mut, budget);
                if bottleneck != bottleneck2 || bottleneck != bottleneck3 {
                    panic!("Bottlenecks are not equal for {}, bottleneck Berman {}, bottleneck Punnen {}, bottleneck edge_elm {}", path, bottleneck, bottleneck2, bottleneck3);
                }
            }
        }
    }
}