use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use rust_jwt_authentication::{handlers::healthcheck::healthcheck, configurations::app_configuration::AppConfiguration, middlewares::auth_middleware::AuthMiddlewareInitializer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let app_configurations = AppConfiguration::get_configuration().expect("Unable to build configuration");
    let db_connection_string = app_configurations.database_configuration.get_connection_string();
    
    HttpServer::new(|| {
        App::new()
            .wrap(AuthMiddlewareInitializer)
            .wrap(Logger::default())
            .route("/health-check", web::get().to(healthcheck))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
