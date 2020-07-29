use std::io::Write;
use std::fs::File;
use std::io;

//use serde::{Serialize, Deserialize};
use actix_multipart::Multipart;
use actix_web::{web, post, get, delete, App, Error, HttpServer, Responder, HttpResponse, HttpRequest};
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



// constantes
static DATA_PREFIX: &str = "./data";
static DATA_SUFFIX: &str = ".data";


#[get("/graph")]
async fn get_default_graph() -> impl Responder {
    let g = Graph::new(vec![1, 2, 3, 4], vec![(1,2), (1,3), (2,3), (4,4), (2,4)]);
    let res = ResponseGraph::from_data(&g);
    let j =  serde_json::to_string(&res).unwrap();
    HttpResponse::Ok().body(j)
}

#[get("/graphs")]
async fn get_all_graphs() -> Result<HttpResponse, Error> {
    let data_dir = std::fs::read_dir(format!("{}/", DATA_PREFIX)).unwrap();
    let mut data_files = vec![];
    for path in data_dir {
        data_files.push(
            path.unwrap().file_name()
                .into_string().unwrap()
                .strip_suffix(DATA_SUFFIX).unwrap()
                .to_string()
        );
    }
    let res = serde_json::to_string(&data_files).unwrap();
    Ok(HttpResponse::Ok().body( res ) )
}

// increase security in this method
// TODO :: if the file exists, it should return error
// TODO :: if there is no space available, it should return error
// TODO :: this method should return value after creating file
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
                format!("{}/{}{}", DATA_PREFIX,  sanitize_filename::sanitize(&filename), DATA_SUFFIX)
            } else {
                format!("{}/temporary{}", DATA_PREFIX, DATA_SUFFIX)
            };
            // File::create is blocking operation, use threadpool
            let mut f: File = web::block(|| std::fs::File::create(filepath))
                .await
                .unwrap();
            file_created = true;
            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                // filesystem operations are blocking, we have to use threadpool
                f = web::block(move || f.write_all(&data).map(|_| f)).await?;
            }
        } else {
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                let new_name = std::str::from_utf8(&data).unwrap();
                if file_created {
                    std::fs::rename(
                        format!("{}/temporary{}", DATA_PREFIX, DATA_SUFFIX),
                        format!("{}/{}{}", DATA_PREFIX, new_name, DATA_SUFFIX)
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

#[delete("/graph")]
async fn delete_graph() -> Result<HttpResponse, Error> {

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
            .service(get_all_graphs)

    ).bind("localhost:3000")?
        .run()
        .await
}
