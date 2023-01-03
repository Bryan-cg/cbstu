#[cfg(test)]
mod tests {
    use crate::algorithms::constrained_bottleneck_spanning_tree::berman::Berman;
    use crate::io::input_handler::InputHandler;

    #[test]
    fn test_easy() {
        let graph = InputHandler::read("data/abilene--D-B-M-N-C-A-N-N_network12_15.json");
        let neg_graph = graph.negative_weights();
        let (_, _, bottleneck_big_budget) = Berman::run(&neg_graph, 10000.0);
        let (_, _, bottleneck_small_budget) = Berman::run(&neg_graph, 100.0);
        assert_eq!(bottleneck_big_budget, -26.0);
        assert_eq!(bottleneck_small_budget, -14.0);
    }
}