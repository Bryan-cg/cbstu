use std::env;
use log::info;

mod datastructures;
mod algorithms;

fn main() {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    info!("Starting program");
}
