#![allow(warnings)]

use std::{env, fs};
use std::time::Instant;
use log::info;
use rand::Rng;
use crate::algorithms::constrained_bottleneck_spanning_tree::berman::Berman;
use crate::algorithms::constrained_bottleneck_spanning_tree::edge_elimination::EdgeElimination;
use crate::algorithms::constrained_bottleneck_spanning_tree::punnen::Punnen;
use crate::algorithms::min_bottleneck_spanning_tree::camerini::MBST;
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

fn cli() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = args.get(1).expect("First CLI argument needs to be path to input file");
    let algorithm = args.get(2).expect("Second CLI argument needs to be algorithm").as_str();
    let budget = args.get(3).expect("Third CLI argument needs to be budget").parse::<f64>().expect("Budget needs to be a number");
    let mut graph = InputHandler::read_mut(input_file_path);
    graph.inverse_weights();
    info!("Solving with algorithm {}", algorithm);
    let now = Instant::now();
    let (_, _, bottleneck) = match algorithm {
        "punnen" => Punnen::run(&graph, budget),
        "berman" => Berman::run(&graph, budget),
        "edge_elimination" => EdgeElimination::run(&graph, budget),
        _ => panic!("Algorithm not supported"),
    };
    info!("Algorithm took {} ms", (now.elapsed().as_nanos() as f64 / 1_000_000.0));
    info!("Bottleneck: {}", bottleneck);
}

fn benchmark() {
    //let paths = fs::read_dir("data").unwrap();
    //let paths = fs::read_dir("C:/Users/Bryan Coulier/Documents/PhD/Network upgrading problem/scripts/data_steiner/WRP4").unwrap();
    //let paths = fs::read_dir("/mnt/c/Users/Bryan Coulier/Documents/PhD/Network upgrading problem/scripts/dimacs_network_dataset/p-hat").unwrap();
    let paths = fs::read_dir("C:/Users/Bryan Coulier/Documents/PhD/Network upgrading problem/scripts/dimacs_network_dataset/C").unwrap();
    let mut times = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();
        if path.ends_with(".json") {
            let budget = rand::thread_rng().gen_range(20.0..100.0);
            let mut graph_mut = InputHandler::read_mut(path);
            graph_mut.inverse_weights();
            info!("Solving with algorithm Berman");
            let start_berman = Instant::now();
            let (st1, _, bottleneck1) = Berman::run(&graph_mut, budget);
            let end_berman = start_berman.elapsed().as_nanos() as f64 / 1_000_000.0;
            info!("Solving with algorithm Punnen");
            let start_punnen = Instant::now();
            let (st2, _, bottleneck2) = Punnen::run(&graph_mut, budget);
            let end_punnen = start_punnen.elapsed().as_nanos() as f64 / 1_000_000.0;
            info!("Solving with algorithm EE");
            let start_edge_elimination = Instant::now();
            let (st3, _, bottleneck3) = EdgeElimination::run(&graph_mut, budget);
            let end_edge_elimination = start_edge_elimination.elapsed().as_nanos() as f64 / 1_000_000.0;
            debug_assert!(st1.unwrap().is_spanning_tree());
            debug_assert!(st2.unwrap().is_spanning_tree());
            debug_assert!(st3.unwrap().is_spanning_tree());
            assert_eq!(bottleneck1, bottleneck2);
            assert_eq!(bottleneck1, bottleneck3);
            times.push((graph_mut.nodes().len(), graph_mut.edges().len(), end_berman, end_punnen, end_edge_elimination, bottleneck1));
        }
    }
    //order times by nodes
    times.sort_by(|a, b| a.0.cmp(&b.0));
    println!("Results:");
    println!("Nodes \t Edges \t Berman[ms] \t Punnen[ms] \t EE[ms] \t Bottleneck");
    for (v, e, berman, punnen, edge_el, bottleneck) in &times {
        println!("{} \t {} \t {:.5} \t {:.5} \t {:.5} \t {}", v, e, berman, punnen, edge_el, -bottleneck);
    }
}
