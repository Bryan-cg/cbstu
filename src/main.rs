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

///Data
//http://sndlib.zib.de/home.action
//https://steinlib.zib.de/

fn main() {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    info!("Starting program");
    cli();
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
