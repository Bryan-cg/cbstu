#[cfg(test)]
mod tests {
    use crate::algorithms::constrained_bottleneck_spanning_tree::berman::Berman;
    use crate::algorithms::constrained_bottleneck_spanning_tree::punnen::Punnen;
    use crate::io::input_handler::InputHandler;

    #[test]
    fn test_easy_instance() {
        let graph = InputHandler::read("data/abilene--D-B-M-N-C-A-N-N_network12_15.json");
        let neg_graph = graph.negative_weights();
        let (_, _, bottleneck_big_budget_berman) = Berman::run(&neg_graph, 10000.0);
        let (_, _, bottleneck_small_budget_berman) = Berman::run(&neg_graph, 100.0);
        let (_, _, bottleneck_big_budget_punnen) = Punnen::run(&neg_graph, 10000.0);
        let (_, _, bottleneck_small_budget_punnen) = Punnen::run(&neg_graph, 100.0);
        assert_eq!(bottleneck_big_budget_berman, -26.0);
        assert_eq!(bottleneck_small_budget_berman, -14.0);
        assert_eq!(bottleneck_big_budget_punnen, -26.0);
        assert_eq!(bottleneck_small_budget_punnen, -14.0);
    }

    #[test]
    fn test_difficult_instance1() {
        let graph = InputHandler::read("data/ta2--D-B-E-N-C-A-N-N_network65_108.json");
        let neg_graph = graph.negative_weights();
        let (_, _, bottleneck) = Berman::run(&neg_graph, 300.0);
        let (_, _, bottleneck2) = Punnen::run(&neg_graph, 300.0);
        assert_eq!(bottleneck, -13.0);
        assert_eq!(bottleneck2, -13.0);
    }

    #[test]
    fn test_difficult_instance2() {
        let graph = InputHandler::read("data/wrp4-11_network123_233.json");
        let neg_graph = graph.negative_weights();
        let (_, _, bottleneck_small) = Berman::run(&neg_graph, 100.0);
        let (_, _, bottleneck_small2) = Punnen::run(&neg_graph, 100.0);
        let (_, _, bottleneck_mid) = Berman::run(&neg_graph, 700.0);
        let (_, _, bottleneck_mid2) = Punnen::run(&neg_graph, 700.0);
        let (_, _, bottleneck_big) = Berman::run(&neg_graph, 10000.0);
        let (_, _, bottleneck_big2) = Punnen::run(&neg_graph, 10000.0);
        assert_eq!(bottleneck_small, -2.0);
        assert_eq!(bottleneck_small2, -2.0);
        assert_eq!(bottleneck_mid, -8.0);
        assert_eq!(bottleneck_mid2, -8.0);
        assert_eq!(bottleneck_big, -9.0);
        assert_eq!(bottleneck_big2, -9.0);
    }
}