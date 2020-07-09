//use serde::{Serialize, Deserialize};
use serde_json::Result;
//use actix_web::{get, web, App, HttpServer, Responder};
//use log::{info, warn, debug, error};

//mod mandelbrot;
mod structure;
mod response;
//use structure::node::Node;
use structure::graph::Graph;
use response::graph::Graph as ResponseGraph;

fn main() {
    let g = Graph::new(vec![1, 2, 3, 4], vec![(1,2), (1,3), (2,4), (4,4), (2,4)]);
    let res = ResponseGraph::from_data(&g);
    let j =  serde_json::to_string(&res).unwrap();
    println!("{}", j);
}
