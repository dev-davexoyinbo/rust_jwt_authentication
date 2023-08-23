use std::time::Duration;

use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use rust_jwt_authentication::{
    auth::{
        self,
        middlewares::{
            auth_middleware::AuthMiddlewareInitializer,
            require_auth_middleware::RequireAuthMiddlewareInitializer,
        },
    },
    configurations::app_configuration::AppConfiguration,
    handlers::{authenticated_route, healthcheck::healthcheck},
    states::db_state::DBState,
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

    println!("DB connection: {}", db_connection_string);
    println!("App configuration: {:?}", app_configurations);

    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .connect(&db_connection_string)
        .await
        .expect("Unable to connect to the postgres database");

    let db_state = web::Data::new(DBState { pool });

    HttpServer::new(move || {
        App::new()
            .app_data(db_state.clone())
            .wrap(AuthMiddlewareInitializer)
            .wrap(Logger::default())
            .configure(auth::handlers::auth_config)
            .route("/health-check", web::get().to(healthcheck))
            .service(
                web::scope("api")
                    .wrap(RequireAuthMiddlewareInitializer)
                    .route("authenticated-route", web::get().to(authenticated_route)),
            )
    })
    .bind((app_configurations.app_config.host, app_configurations.app_config.port))?
    .run()
    .await
}
