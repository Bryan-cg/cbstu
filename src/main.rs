#![allow(warnings)]
use std::{env, fs};
use std::time::Instant;
use log::info;
use crate::algorithms::constrained_bottleneck_spanning_tree::berman::Berman;
use crate::algorithms::constrained_bottleneck_spanning_tree::edge_elimination::EdgeElimination;
use crate::algorithms::constrained_bottleneck_spanning_tree::punnen::Punnen;
use crate::io::input_handler::InputHandler;

mod datastructures;
mod algorithms;
mod tests_functions;
mod io;
//http://sndlib.zib.de/home.action
//https://steinlib.zib.de/
fn main() {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    info!("Starting program");
    test_all();
    info!("Finished");
}

fn cli() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = args.get(1).expect("First CLI argument needs to be path to input file");
    let algorithm = args.get(2).expect("Second CLI argument needs to be algorithm").as_str();
    let budget = args.get(3).expect("Third CLI argument needs to be budget").parse::<f64>().expect("Budget needs to be a number");
    let graph = InputHandler::read(input_file_path);
    let neg_graph = graph.negative_weights();
    let now = Instant::now();
    let (_, _, bottleneck) = match algorithm {
        "punnen" => Punnen::run(&neg_graph, budget),
        "berman" => Berman::run(&neg_graph, budget),
        "edge_elimination" => EdgeElimination::run(&neg_graph, budget),
        _ => panic!("Algorithm not supported"),
    };
    info!("Algorithm took {} ns", now.elapsed().as_nanos());
    info!("Bottleneck: {}", bottleneck);
}

fn test_all() {
    let paths = fs::read_dir("data").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();
        if path.ends_with(".json") {
            let graph = InputHandler::read(path);
            let neg_graph = graph.negative_weights();
            let (_, _, bottleneck) = Berman::run(&neg_graph, 200.0);
            let (_, _, bottleneck2) = Punnen::run(&neg_graph, 200.0);
            let (_, _, bottleneck3) = EdgeElimination::run(&neg_graph, 200.0);
            if bottleneck != bottleneck2 || bottleneck != bottleneck3 {
                panic!("Bottlenecks are not equal for {}, bottleneck Berman {}, bottleneck Punnen {}, bottleneck edge_elm {}", path, bottleneck, bottleneck2, bottleneck3);
            }
        }
    }
}
