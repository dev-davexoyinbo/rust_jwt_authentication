use std::collections::HashMap;

use config::{Config, ConfigError, File, FileFormat};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AppConfiguration {
    pub app_config: AppConfig,
    pub database_configuration: DatabaseConfiguration,
    pub jwt_config: JwtConfig,
}

impl AppConfiguration {
    pub fn get_configuration() -> Result<AppConfiguration, ConfigError> {
        let base_path = std::env::current_dir().expect("Failed to determine the current directory");
        let configuration_path = base_path.join("configurations");

        let env_string = std::fs::read_to_string(".env").unwrap_or("".to_string());
        let environment_data: HashMap<String, String> = Config::builder()
            .add_source(File::from_str(&env_string, FileFormat::Ini))
            .build()?
            .try_deserialize()?;

        let handlebars = handlebars::Handlebars::new();
        let template_string = std::fs::read_to_string(configuration_path.join("base.yaml"))
            .expect("Unable to open configuration file");

        let rendered = handlebars
            .render_template(&template_string, &environment_data)
            .expect("Unable to render template");

        let builder = Config::builder()
            .add_source(File::from_str(rendered.as_str(), FileFormat::Yaml).required(true));

        return builder.build()?.try_deserialize();
    }
}

// Database configuration struct
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DatabaseConfiguration {
    postgres_host: String,
    postgres_port: u32,
    postgres_user: String,
    postgres_password: String,
    postgres_db: String,
}

impl DatabaseConfiguration {
    pub fn get_connection_string(&self) -> String {
        return format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_db
        );
    }
}


// Jwt config struct
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct JwtConfig {
    pub secret: String,
    pub expires_in: String,
    pub maxage: u32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}