use std::io::Write;

//use serde::{Serialize, Deserialize};
use actix_multipart::Multipart;
use actix_web::{web, post, get, App, Error, HttpServer, Responder, HttpResponse, HttpRequest};
use actix_cors::Cors;

use futures::{StreamExt, TryStreamExt};
//use log::{info, warn, debug, error};

//mod mandelbrot;
mod structure;
mod response;
mod request;
//use structure::node::Node;
use structure::graph::Graph;
use response::graph::Graph as ResponseGraph;
use request::structures::UploadGraphParams;
use std::fs::File;




// constantes
static DATA_PREFIX: &str = "./data";


#[get("/graph")]
async fn get_default_graph() -> impl Responder {
    let g = Graph::new(vec![1, 2, 3, 4], vec![(1,2), (1,3), (2,3), (4,4), (2,4)]);
    let res = ResponseGraph::from_data(&g);
    let j =  serde_json::to_string(&res).unwrap();
    HttpResponse::Ok().body(j)
}

#[post("/graph")]
async fn upload_graph(mut payload: Multipart) ->  Result<HttpResponse, Error> {
    let mut filename = String::new();
    let mut file_created = false;
    let mut received_name = false;
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let is_file = match content_type.get_filename() {
            Some(_) => true,
            _ => false,
        };
        if is_file {
            let filepath = if received_name {
                format!("{}/{}.data", DATA_PREFIX,  sanitize_filename::sanitize(&filename))
            } else {
                format!("{}/temporary.data", DATA_PREFIX,)
            };

            println!("filepath : {}", filepath);
            // File::create is blocking operation, use threadpool
            let mut f: File = web::block(|| std::fs::File::create(filepath))
                .await
                .unwrap();
            file_created = true;
            println!("File created");
            // Field in turn is stream of *Bytes* object
            println!("writing all chunks");
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                // filesystem operations are blocking, we have to use threadpool
                f = web::block(move || f.write_all(&data).map(|_| f)).await?;
            }
        } else {
            println!("getting graphName");
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                let new_name = std::str::from_utf8(&data).unwrap();
                if file_created {
                    std::fs::rename(
                        format!("{}/temporary.data", DATA_PREFIX,),
                        format!("{}/{}.data", DATA_PREFIX, new_name)
                    );
                } else {
                    filename = new_name.to_string();
                    received_name = true;
                }
            }
        }
    }
    Ok(HttpResponse::Ok().into())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::fs::create_dir_all(format!("{}", DATA_PREFIX)).unwrap();
    HttpServer::new(||
        App::new()
            .wrap(
                Cors::new()
                    .supports_credentials().finish()
                )
            .service(get_default_graph)
            .service(upload_graph)

    ).bind("localhost:3000")?
        .run()
        .await
}
