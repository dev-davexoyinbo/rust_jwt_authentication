use actix_web::{web, App, HttpResponse, HttpServer, middleware::Logger};
use env_logger::Env;
use log::{info, error};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default())
        .route(
            "/",
            web::get().to(|| async {
                HttpResponse::Ok().body("This is the response") }),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
