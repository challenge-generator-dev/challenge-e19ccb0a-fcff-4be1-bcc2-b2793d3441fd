//! Configuración de la aplicación.

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

/// Configuración de la aplicación.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// URL de conexión a la base de datos.
    pub database_url: String,
    /// Puerto en el que escucha la aplicación.
    pub port: u16,
}

impl AppConfig {
    /// Carga la configuración desde las variables de entorno.
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();

        Ok(AppConfig {
            database_url: env::var("DATABASE_URL")?,
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
        })
    }

    /// Crea un pool de conexiones a la base de datos.
    pub async fn create_pool(&self) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.database_url)
            .await
    }
}