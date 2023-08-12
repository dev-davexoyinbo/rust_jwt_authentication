use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use rust_jwt_authentication::{
    auth, configurations::app_configuration::AppConfiguration, handlers::healthcheck::healthcheck,
    middlewares::auth_middleware::AuthMiddlewareInitializer,
};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let app_configurations =
        AppConfiguration::get_configuration().expect("Unable to build configuration");

    let db_connection_string = app_configurations
        .database_configuration
        .get_connection_string();

    let pool = PgPoolOptions::new()
        .connect(&db_connection_string)
        .await
        .expect("Unable to connect to the postgres database");

    let db_pool = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .wrap(AuthMiddlewareInitializer)
            .wrap(Logger::default())
            .configure(auth::handlers::auth_config)
            .route("/health-check", web::get().to(healthcheck))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
