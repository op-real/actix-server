use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use listenfd::ListenFd;
use dotenv::dotenv;
use std::env;

mod app;
mod math;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .configure(app::routes)
            .service(index)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind(env::var("BIND_ADDRESS").expect("Binding address must be set"))?
    };

    server.run().await
}
