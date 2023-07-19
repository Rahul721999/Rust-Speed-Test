#![allow(non_snake_case)]
use actix_files::NamedFile;
use actix_web::{Responder, web, App, HttpServer, middleware};
use env_logger::{Env, init_from_env};



///
/// Get the index.html file
/// 
async fn index() -> impl Responder{
    NamedFile::open_async("./static/index.html").await.expect("failed to index file")
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    // load env var
    init_from_env(Env::new().default_filter_or("info"));
    log::info!("🚀 Starting server at http://localhost:8080");

    HttpServer::new(||{
        App::new()
         .service(web::resource("/").to(index))
         .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
