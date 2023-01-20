#![allow(warnings)]

use std::{env, fs};
use std::time::Instant;
use log::info;
use rand::Rng;
use crate::algorithms::constrained_bottleneck_spanning_tree::berman::Berman;
use crate::algorithms::constrained_bottleneck_spanning_tree::edge_elimination::EdgeElimination;
use crate::algorithms::constrained_bottleneck_spanning_tree::ee_fast::EE;
use crate::algorithms::constrained_bottleneck_spanning_tree::punnen::Punnen;
use crate::algorithms::min_bottleneck_spanning_tree::camerini::MBST;
use crate::algorithms::min_sum_spanning_tree::kruskal::{CalculationType, Kruskal};
use crate::algorithms::util::Util;
use crate::datastructures::graph::mutable_graph::MutableGraph;
use crate::io::input_handler::InputHandler;

mod datastructures;
mod algorithms;
mod tests_functions;
mod io;

///Data
//http://sndlib.zib.de/home.action
//https://steinlib.zib.de/
//https://dimacs11.zib.de/downloads.html
//https://networkrepository.com/dimacs.php

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Starting program");
    benchmark();
    info!("Finished");
}

// fn cli() {
//     let args: Vec<String> = env::args().collect();
//     let input_file_path = args.get(1).expect("First CLI argument needs to be path to input file");
//     let algorithm = args.get(2).expect("Second CLI argument needs to be algorithm").as_str();
//     let budget = args.get(3).expect("Third CLI argument needs to be budget").parse::<f64>().expect("Budget needs to be a number");
//     let mut graph = InputHandler::read_mut(input_file_path);
//     graph.inverse_weights();
//     info!("Solving with algorithm {}", algorithm);
//     let now = Instant::now();
//     let (_, _, bottleneck) = match algorithm {
//         "punnen" => Punnen::run(&graph, budget),
//         "berman" => Berman::run(&graph, budget),
//         "edge_elimination" => EdgeElimination::run(&graph, budget),
//         _ => panic!("Algorithm not supported"),
//     };
//     info!("Algorithm took {} ms", (now.elapsed().as_nanos() as f64 / 1_000_000.0));
//     info!("Bottleneck: {}", bottleneck);
// }

fn profile() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = args.get(1).expect("First CLI argument needs to be path to input file");
    let budget = args.get(2).expect("Second CLI argument needs to be budget").parse::<f64>().expect("Budget needs to be a number");
    info!("Starting benchmarking for {}", input_file_path);
    let mut graph_mut = InputHandler::read_mut(input_file_path);
    let (mut graph, mut duplicated_graph) = preprocessing(graph_mut);
    let start_ee = Instant::now();
    //let (st4, _, bottleneck4) = EE::run(&mut graph, &mut duplicated_graph, budget);
    let (st4, _, bottleneck4) = EE::run_test(duplicated_graph, budget);
    let end_ee = start_ee.elapsed().as_nanos() as f64 / 1_000_000.0;
    //info!("EE Fast took {} ms", end_ee);
}

fn mbst_test() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = args.get(1).expect("First CLI argument needs to be path to input file");
    let budget = args.get(2).expect("Second CLI argument needs to be budget").parse::<f64>().expect("Budget needs to be a number");
    info!("Starting benchmarking for {}", input_file_path);
    let mut graph_mut = InputHandler::read_mut(input_file_path);
    let (mut graph, mut duplicated_graph) = preprocessing(graph_mut);
    info!("MBST");
    let start_sort = Instant::now();
    let (st1, bottleneck1) = MBST::run_mutable(&mut duplicated_graph);
    let end_sort = start_sort.elapsed().as_nanos() as f64 / 1_000_000.0;
    info!("MBST took {} ms", end_sort);
    let mut graph_mut = InputHandler::read_mut(input_file_path);
    let (mut graph, mut duplicated_graph) = preprocessing(graph_mut);
    info!("Kruskal");
    let start_ee = Instant::now();
    let (st4, _, bottleneck4) = Kruskal::run_mutable(&mut duplicated_graph, CalculationType::Weight, None);
    let end_ee = start_ee.elapsed().as_nanos() as f64 / 1_000_000.0;
    info!("Kruskal took {} ms", end_ee);
}

fn benchmark() {
    let args: Vec<String> = env::args().collect();
    let paths_str = args.get(1).expect("First CLI argument needs to be path to directory");
    let paths = fs::read_dir(paths_str).unwrap();
    let budget = args.get(2).expect("Second CLI argument needs to be budget").parse::<f64>().expect("Budget needs to be a number");
    info!("Starting benchmarking in directory {}", paths_str);
    let mut times = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();
        if path.ends_with(".json") {
            //let budget = rand::thread_rng().gen_range(100.0..1000.0);
            let mut graph_mut = InputHandler::read_mut(path);
            let (mut graph, mut duplicated_graph) = preprocessing(graph_mut);
            info!("Solving with algorithm Berman");
            let start_berman = Instant::now();
            let (st1, _, bottleneck1) = Berman::run(&mut duplicated_graph, budget);
            let end_berman = start_berman.elapsed().as_nanos() as f64 / 1_000_000.0;

            graph_mut = InputHandler::read_mut(path);
            let (mut graph, mut duplicated_graph) = preprocessing(graph_mut);
            info!("Solving with algorithm Punnen");
            let start_punnen = Instant::now();
            let (st2, _, bottleneck2) = Punnen::run(&mut duplicated_graph, budget);
            let end_punnen = start_punnen.elapsed().as_nanos() as f64 / 1_000_000.0;
            //
            // graph_mut = InputHandler::read_mut(path);
            // let (mut graph, mut duplicated_graph) = preprocessing(graph_mut);
            // info!("Solving with algorithm EE");
            // let start_edge_elimination = Instant::now();
            // let (st3, _, bottleneck3, mut bin) = EdgeElimination::run(&mut duplicated_graph, budget);
            // let end_edge_elimination = start_edge_elimination.elapsed().as_nanos() as f64 / 1_000_000.0;

            info!("Solving with algorithm EE Fast");
            graph_mut = InputHandler::read_mut(path);
            let (mut graph, mut duplicated_graph) = preprocessing(graph_mut);
            let start_ee = Instant::now();
            let (st4, _, bottleneck4, mut bin) = EE::run_test_bin(duplicated_graph, budget);
            let end_ee = start_ee.elapsed().as_nanos() as f64 / 1_000_000.0;

            debug_assert!(st1.unwrap().is_spanning_tree());
            debug_assert!(st2.unwrap().is_spanning_tree());
            // debug_assert!(st3.unwrap().is_spanning_tree());
            debug_assert!(st4.unwrap().is_spanning_tree());
            assert_eq!(bottleneck1, bottleneck2);
            // assert_eq!(bottleneck1, bottleneck3);
            assert_eq!(bottleneck1, bottleneck4);
            //get fastest algorithm
            let list = vec![end_berman, end_punnen, end_ee];
            let fastest = list.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let fastest_name = match fastest {
                x if x == &end_berman => "BERMAN",
                x if x == &end_punnen => "PUNNEN",
                // x if x == &end_edge_elimination => "edge_el",
                x if x == &end_ee => "EE",
                _ => panic!("Something went wrong"),
            };
            times.push((graph.nodes().len(), graph.edges().len(), end_berman, end_punnen, end_ee, bottleneck4, fastest_name));
            info!("Garbage size: {}", bin.len());
            bin.clear();
        }
    }
    //order times by nodes
    times.sort_by(|a, b| a.0.cmp(&b.0));
    println!("Results:");
    println!("Nodes \t Edges \t Berman[ms] \t Punnen[ms] \t EE \t Bottleneck \t Fastest");
    for (v, e, berman, punnen, end_EE, bottleneck, fastest) in &times {
        println!("{} \t {} \t {:.3} \t {:.3} \t {:.3} \t {} \t {}", v, e, berman, punnen, end_EE, -bottleneck, fastest);
    }
}

fn preprocessing(mut graph: MutableGraph) -> (MutableGraph, MutableGraph) {
    info!("Preprocessing");
    graph.inverse_weights();
    let duplicated_graph = Util::duplicate_edges_mut(&graph);
    (graph, duplicated_graph)
}

