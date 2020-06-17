//use serde::{Serialize, Deserialize};
//use actix_web::{get, web, App, HttpServer, Responder};
//use log::{info, warn, debug, error};

mod structure;

use crate::structure::node::Node;


//#[get("/{id}/{name}/index.html")]
//async fn index(info: web::Path<(u32, String)>) -> impl Responder {
//    println!("info req {:?}", info);
//    info!("/:id/:name called with name = {} | id = {}", info.1, info.0);
//    format!("Hello {}! id:{}", info.1, info.0)
//}

//#[actix_rt::main]
//async fn main() -> std::io::Result<()> {
//    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
//    HttpServer::new(|| App::new().service(index))
//        .bind("127.0.0.1:8080")?
//        .run()
//        .await
//}

fn main() {
    let mut node = Node::new(32);
    let node2 = Node::new(13);
    let node3 = Node::new(15);
    node.connect_to_node(&node2, 1);
    println!("has neighbors test {:?}", node.has_neighbor(&node2));
    println!("is neighbor of neighbors test {:?}", node2.is_neighbor_of(&node));
    println!("created node   {:?}", node);
    println!("created node 2 {:?}", node2);
    println!("created node 3 {:?}", node3);
}
