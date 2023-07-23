#![allow(non_snake_case)]
use actix_files::NamedFile;
use actix_web::{Responder, web, App, HttpServer, middleware, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use env_logger::{Env, init_from_env};
mod server;
use self::server::MyWebSocket;



// Get the index.html file
async fn index() -> impl Responder{
    NamedFile::open_async("./static/index.html").await.expect("failed to index file")
}

// WebSocket handshake and start 'MyWebSocket' actor....
async fn websocket(req : HttpRequest, stream : web::Payload) -> Result<HttpResponse, Error>{
    ws::start(MyWebSocket::new(), &req,stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    // load env var
    init_from_env(Env::new().default_filter_or("info"));
    log::info!("🚀 Starting server at http://localhost:8000");

    HttpServer::new(||{
        App::new()
         .service(web::resource("/").to(index))
         .service(web::resource("/ws").route(web::get().to(websocket)))
         .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8000))?
    .run()
    .await

}
