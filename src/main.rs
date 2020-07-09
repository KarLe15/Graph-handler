//use serde::{Serialize, Deserialize};
use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
//use log::{info, warn, debug, error};

//mod mandelbrot;
mod structure;
mod response;
//use structure::node::Node;
use structure::graph::Graph;
use response::graph::Graph as ResponseGraph;

#[get("/graph")]
async fn get_default_graph() -> impl Responder {
    let g = Graph::new(vec![1, 2, 3, 4], vec![(1,2), (1,3), (2,3), (4,4), (2,4)]);
    let res = ResponseGraph::from_data(&g);
    let j =  serde_json::to_string(&res).unwrap();
    HttpResponse::Ok().body(j)
}



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .wrap(
                Cors::new()
                    .supports_credentials().finish()
                )
            .service(get_default_graph)

    ).bind("localhost:3000")?
        .run()
        .await
}
