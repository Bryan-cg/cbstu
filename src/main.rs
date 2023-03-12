#![allow(warnings)]

use std::{env, fs};
use std::fmt::format;
use std::process::exit;
use std::time::Instant;
use log::{info, warn};
use rand::Rng;
use crate::algorithms::constrained_bottleneck_spanning_tree::berman::Berman;
use crate::algorithms::constrained_bottleneck_spanning_tree::edge_elimination::EdgeEliminationOld;
use crate::algorithms::constrained_bottleneck_spanning_tree::fast_edge_elimination::FastEdgeElimination;
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
    compare_performance_cbstu();
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
    let (st4, _, bottleneck4, mut bin) = FastEdgeElimination::run(duplicated_graph, budget);
    let end_ee = start_ee.elapsed().as_nanos() as f64 / 1_000_000.0;
    info!("EE Fast took {} ms", end_ee);
    bin.clear();
}

fn compare_performance_mbst() {
    let args: Vec<String> = env::args().collect();
    let paths_str = args.get(1).expect("First CLI argument needs to be path to directory");
    let paths = fs::read_dir(paths_str).unwrap();
    //let budget = args.get(2).expect("Second CLI argument needs to be budget").parse::<f64>().expect("Budget needs to be a number");
    info!("Starting benchmarking in directory {}", paths_str);
    let mut times = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();
        if path.ends_with(".json") {
            info!("Preprocessing");
            let mut graph_mut = InputHandler::read_mut(path);
            let (mut graph, mut duplicated_graph) = preprocessing(graph_mut);
            info!("Solving with MBST");
            let now = Instant::now();
            let (st, bottleneck) = MBST::run(&mut duplicated_graph);
            let time_mbst = now.elapsed().as_nanos() as f64 / 1_000_000.0;
            info!("MBST took {} ms", time_mbst);
            let mut graph_mut = InputHandler::read_mut(path);
            let (mut graph, mut duplicated_graph) = preprocessing(graph_mut);
            let now = Instant::now();
            let (st2, _, bottleneck2) = Kruskal::run(&mut duplicated_graph, CalculationType::Weight);
            let time_st = now.elapsed().as_nanos() as f64 / 1_000_000.0;
            info!("MST took {} ms", time_st);
            assert_eq!(bottleneck, bottleneck2);
            let path_name_part = String::from(path.split(".mtx").next().unwrap());
            let path_name = String::from(path_name_part.split("combined/").last().unwrap());
            times.push((graph.nodes().len(), graph.edges().len(), time_mbst, time_st, path_name));
        }
    }
    times.sort_by(|a, b| a.0.cmp(&b.0));
    println!("N \t E \t MBST \t MST \t File");
    for (nodes, edges, time_mbst, time_st, path_name) in times {
        println!("{} \t {} \t {} \t {} \t {}", path_name, nodes, edges, time_mbst, time_st);
    }
}

fn compare_performance_cbst() {
    let args: Vec<String> = env::args().collect();
    let paths_str = args.get(1).expect("First CLI argument needs to be path to directory");
    let paths = fs::read_dir(paths_str).unwrap();
    //let budget = args.get(2).expect("Second CLI argument needs to be budget").parse::<f64>().expect("Budget needs to be a number");
    info!("Starting benchmarking in directory {}", paths_str);
    let mut times = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();
        if path.ends_with(".json") {
            info!("Solving MBST & MST");
            let mut graph = InputHandler::read_mut(path);
            graph.inverse_weights();
            let (mbst, bottleneck_mbst) = MBST::run(&mut graph);
            let cost_mbst = mbst.unwrap().calculate_total_cost();
            graph = InputHandler::read_mut(path);
            graph.inverse_weights();
            let (_, cost_mst, _) = Kruskal::run(&mut graph, CalculationType::Cost);
            let budget = rand::thread_rng().gen_range(cost_mst..cost_mbst) as u64 as f64;

            graph = InputHandler::read_mut(path);
            graph.inverse_weights();
            let num_nodes = graph.nodes().len();
            let num_edges = graph.edges().len();
            info!("Solving with algorithm Berman");
            let start_berman = Instant::now();
            let (st1, cost_berman, bottleneck1) = Berman::run(&mut graph, budget);
            let end_berman = start_berman.elapsed().as_nanos() as f64 / 1_000_000.0;

            graph = InputHandler::read_mut(path);
            graph.inverse_weights();
            info!("Solving with algorithm Punnen");
            let start_punnen = Instant::now();
            let (st2, cost_punnen, bottleneck2) = Punnen::run(&mut graph, budget);
            let end_punnen = start_punnen.elapsed().as_nanos() as f64 / 1_000_000.0;

            graph = InputHandler::read_mut(path);
            graph.inverse_weights();
            info!("Solving with algorithm EE Fast");
            let start_ee = Instant::now();
            let (st4, cost_ee, bottleneck4, mut bin) = FastEdgeElimination::run(graph, budget);
            let end_ee = start_ee.elapsed().as_nanos() as f64 / 1_000_000.0;

            assert!(st1.unwrap().is_spanning_tree());
            assert!(st2.unwrap().is_spanning_tree());
            assert!(st4.unwrap().is_spanning_tree());
            assert_eq!(bottleneck1, bottleneck2);
            assert_eq!(bottleneck1, bottleneck4);
            let list = vec![end_berman, end_punnen, end_ee];
            let fastest = list.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let fastest_name = match fastest {
                x if x == &end_berman => "BERMAN",
                x if x == &end_punnen => "PUNNEN",
                x if x == &end_ee => "EE",
                _ => panic!("Something went wrong"),
            };
            //split path name on "mtx" and take first part
            let path_name_part = String::from(path.split(".mtx").next().unwrap());
            let path_name = String::from(path_name_part.split("combined/").last().unwrap());
            times.push((path_name, num_nodes, num_edges, budget, end_berman, end_punnen, end_ee, bottleneck4, bottleneck_mbst, fastest_name, cost_berman, cost_punnen, cost_ee, cost_mbst));
            bin.clear();
        }
    }
    //order times by nodes
    times.sort_by(|a, b| a.1.cmp(&b.1));
    println!("Results:");
    println!("File \t Nodes \t Edges \t Budget \t Berman[ms] \t Punnen[ms] \t EE \t Bottleneck \t Bottleneck_mbst \t Fastest");
    for (path_name, v, e, budget, berman, punnen, end_EE, bottleneck, bottleneck_mbst, fastest,  cost_berman, cost_punnen, cost_ee, cost_mbst) in &times {
        println!("{} \t {} \t {} \t {} \t {:.3} \t {:.3} \t {:.3} \t {} \t {} \t {}", path_name, v, e, budget, berman, punnen, end_EE, -bottleneck, -bottleneck_mbst, fastest);
    }
    //write out to csv file
    let mut wtr = csv::Writer::from_path("results_cbst.csv").unwrap();
    wtr.write_record(&["Path", "Nodes", "Edges", "Budget", "Berman", "Punnen", "EE", "Bottleneck", "Bottleneck MBST", "Cost", "Cost MBST"]).unwrap();
    //write records with format only 3 digits after comma
    for (path_name, v, e, budget, berman, punnen, end_EE, bottleneck, bottleneck_mbst, fastest,  cost_berman, cost_punnen, cost_ee, cost_mbst) in &times {
        wtr.write_record(&[
            path_name,
            &v.to_string(),
            &e.to_string(),
            &format!("{}", budget),
            &format!("{:.3}", berman),
            &format!("{:.3}", punnen),
            &format!("{:.3}", end_EE),
            &format!("{}", -bottleneck),
            &format!("{}", -bottleneck_mbst),
            &format!("{}", cost_berman),
            &format!("{}", cost_mbst),
        ]).unwrap();
    }
    wtr.flush().unwrap();
}

fn compare_performance_cbstu() {
    let args: Vec<String> = env::args().collect();
    let paths_str = args.get(1).expect("First CLI argument needs to be path to directory");
    let paths = fs::read_dir(paths_str).unwrap();
    //let budget = args.get(2).expect("Second CLI argument needs to be budget").parse::<f64>().expect("Budget needs to be a number");
    info!("Starting benchmarking in directory {}", paths_str);
    let mut times = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();
        if path.ends_with(".json") {
            info!("Solving MBST");
            let mut graph_mut = InputHandler::read_mut(path);
            let (mut graph, mut duplicated_graph) = preprocessing(graph_mut);
            let (mbst, bottleneck_mbst) = MBST::run(&mut duplicated_graph);
            let cost_mbst = mbst.unwrap().calculate_total_cost();
            let budget = rand::thread_rng().gen_range(100.0..cost_mbst) as u64 as f64;

            graph_mut = InputHandler::read_mut(path);
            let (mut graph, mut duplicated_graph) = preprocessing(graph_mut);
            info!("Solving with algorithm Berman");
            let start_berman = Instant::now();
            let (st1, cost_berman, bottleneck1) = Berman::run(&mut duplicated_graph, budget);
            let end_berman = start_berman.elapsed().as_nanos() as f64 / 1_000_000.0;

            if -bottleneck_mbst < -bottleneck1 {
                eprintln!("Bottleneck capacity MBST is smaller than bottleneck capacity Berman");
                exit(1);
            } else {
                println!("MBST {} - Berman {}", -bottleneck_mbst, -bottleneck1)
            }

            graph_mut = InputHandler::read_mut(path);
            let (mut graph, mut duplicated_graph) = preprocessing(graph_mut);
            info!("Solving with algorithm Punnen");
            let start_punnen = Instant::now();
            let (st2, cost_punnen, bottleneck2) = Punnen::run(&mut duplicated_graph, budget);
            let end_punnen = start_punnen.elapsed().as_nanos() as f64 / 1_000_000.0;

            graph_mut = InputHandler::read_mut(path);
            let (mut graph, mut duplicated_graph) = preprocessing(graph_mut);
            info!("Solving with algorithm EE Fast");
            let start_ee = Instant::now();
            let (st4, cost_ee, bottleneck4, mut bin) = FastEdgeElimination::run(duplicated_graph, budget);
            let end_ee = start_ee.elapsed().as_nanos() as f64 / 1_000_000.0;

            let num_upgrades = st1.unwrap().number_of_edges_upgraded();

            //debug_assert!(st1.unwrap().is_spanning_tree());
            //debug_assert!(st2.unwrap().is_spanning_tree());
            //debug_assert!(st4.unwrap().is_spanning_tree());
            assert_eq!(bottleneck1, bottleneck2);
            assert_eq!(bottleneck1, bottleneck4);
            let list = vec![end_berman, end_punnen, end_ee];
            let fastest = list.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let fastest_name = match fastest {
                x if x == &end_berman => "BERMAN",
                x if x == &end_punnen => "PUNNEN",
                x if x == &end_ee => "EE",
                _ => panic!("Something went wrong"),
            };
            //split path name on "mtx" and take first part
            let path_name_part = String::from(path.split(".mtx").next().unwrap());
            let path_name = String::from(path_name_part.split("combined/").last().unwrap());
            times.push((graph.nodes().len(), graph.edges().len(), end_berman, end_punnen, end_ee, bottleneck4, bottleneck_mbst, budget, fastest_name, path_name, cost_berman, cost_mbst, num_upgrades));
            bin.clear();
        }
    }
    //order times by nodes
    times.sort_by(|a, b| a.0.cmp(&b.0));
    println!("Results:");
    println!("Nodes \t Edges \t Berman[ms] \t Punnen[ms] \t EE \t Bottleneck \t Bottleneck MBST \t Budget \t Fastest \t File");
    for (v, e, berman, punnen, end_EE, bottleneck, bottleneck_mbst, budget, fastest, path_name, cost_berman, cost_mbst, num_upgrades) in &times {
        println!("{} \t {} \t {:.3} \t {:.3} \t {:.3} \t {} \t {} \t {} \t {} \t {}", v, e, berman, punnen, end_EE, -bottleneck, -bottleneck_mbst, budget, fastest, path_name);
    }
    //write out to csv file
    let mut wtr = csv::Writer::from_path("results_cbstu.csv").unwrap();
    wtr.write_record(&["Path", "Nodes", "Edges", "Budget", "Berman", "Punnen", "EE", "Bottleneck", "Bottleneck MBST", "Cost", "Cost MBST", "Upgrades"]).unwrap();
    //write records with format only 3 digits after comma
    for (v, e, berman, punnen, end_EE, bottleneck, bottleneck_mbst, budget, fastest, path_name, cost_berman, cost_mbst, num_upgrades) in &times {
        wtr.write_record(&[
            path_name,
            &v.to_string(),
            &e.to_string(),
            &format!("{}", budget),
            &format!("{:.3}", berman),
            &format!("{:.3}", punnen),
            &format!("{:.3}", end_EE),
            &format!("{}", -bottleneck),
            &format!("{}", -bottleneck_mbst),
            &format!("{}", cost_berman),
            &format!("{}", cost_mbst),
            &format!("{}", num_upgrades),
        ]).unwrap();
    }
    wtr.flush().unwrap();
}

fn preprocessing(mut graph: MutableGraph) -> (MutableGraph, MutableGraph) {
    info!("Preprocessing");
    graph.inverse_weights();
    let duplicated_graph = Util::duplicate_edges(&graph);
    (graph, duplicated_graph)
}

