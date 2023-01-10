use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use log::info;
use crate::datastructures::graph::edge::Edge;
use crate::datastructures::graph::immutable_graph::ImmutableGraph;
use crate::datastructures::graph::mutable_graph::MutableGraph;
use crate::datastructures::graph::node::Node;

pub struct InputHandler();

impl InputHandler {
    pub fn read(file_name: &str) -> ImmutableGraph {
        info!("Reading file {}", file_name);
        let data = fs::read_to_string(file_name).expect("Unable to read file");
        let json: serde_json::Value = serde_json::from_str(&data).expect("JSON was not well-formatted");
        // Read nodes
        let mut nodes = Vec::new();
        let json_nodes = &json["nodes"];
        for node in json_nodes.as_array().unwrap() {
            let id = node["id"].as_i64().unwrap() as usize;
            let x = node["x"].as_f64().unwrap();
            let y = node["y"].as_f64().unwrap();
            let node = Node::new(id, x, y);
            nodes.push(Rc::new(node));
        }
        // Read edges
        let mut edges = Vec::new();
        let json_edges = &json["links"];
        for edge in json_edges.as_array().unwrap() {
            let either = edge["sourceId"].as_i64().unwrap() as usize;
            let other = edge["targetId"].as_i64().unwrap() as usize;
            let edge = Edge::new(either, other)
                .weight(edge["k"].as_f64().unwrap())
                .upgraded_weight(edge["kBar"].as_f64().unwrap())
                .cost(edge["c"].as_f64().unwrap());
            edges.push(Rc::new(edge));
        }
        info!("Read {} nodes and {} edges", nodes.len(), edges.len());
        ImmutableGraph::new(Rc::new(nodes), edges)
    }

    pub fn read_mut(file_name: &str) -> MutableGraph {
        info!("Reading file {}", file_name);
        let data = fs::read_to_string(file_name).expect("Unable to read file");
        let json: serde_json::Value = serde_json::from_str(&data).expect("JSON was not well-formatted");
        // Read nodes
        let mut nodes = Vec::new();
        let json_nodes = &json["nodes"];
        for node in json_nodes.as_array().unwrap() {
            let id = node["id"].as_i64().unwrap() as usize;
            let x = node["x"].as_f64().unwrap();
            let y = node["y"].as_f64().unwrap();
            let node = Node::new(id, x, y);
            nodes.push(Rc::new(node));
        }
        // Read edges
        let mut edges = Vec::new();
        let json_edges = &json["links"];
        for edge in json_edges.as_array().unwrap() {
            let either = edge["sourceId"].as_i64().unwrap() as usize;
            let other = edge["targetId"].as_i64().unwrap() as usize;
            let edge = Edge::new(either, other)
                .weight(-edge["k"].as_f64().unwrap())
                .upgraded_weight(-edge["kBar"].as_f64().unwrap())
                .cost(edge["c"].as_f64().unwrap());
            edges.push(Rc::new(RefCell::new(edge)));
        }
        info!("Read {} nodes and {} edges", nodes.len(), edges.len());
        MutableGraph::new(Rc::new(nodes), edges)
    }
}