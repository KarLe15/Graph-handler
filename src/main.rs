//use serde::{Serialize, Deserialize};
//use actix_web::{get, web, App, HttpServer, Responder};
//use log::{info, warn, debug, error};

//mod mandelbrot;
mod structure;
//use structure::node::Node;
use structure::graph::Graph;

fn main() {
    let g = Graph::new(vec![1, 2, 3, 4], vec![(1,2), (1,3), (2,4), (4,4), (2,5)]);
    println!("{:?}", g);
}
