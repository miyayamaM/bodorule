use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use api::route::health::build_health_check_routes;
use axum::{extract::State, http::StatusCode, routing::get, Router};
use sqlx::{postgres::PgConnectOptions, PgPool};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = DatabaseConfig {
        host: "localhost".to_string(),
        port: 5432,
        username: "app".to_string(),
        password: "passwd".to_string(),
        database: "app".to_string(),
    };
    let pool = connect_database_with(cfg);

    let registry = registry::AppModule::builder().build();

    let app = Router::new()
        .merge(build_health_check_routes())
        .with_state(registry);

    let address = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    let listener = TcpListener::bind(address).await?;
    println!("listening on {}", address);
    Ok(axum::serve(listener, app).await?)
}

struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
}

impl From<DatabaseConfig> for PgConnectOptions {
    fn from(cfg: DatabaseConfig) -> Self {
        PgConnectOptions::new()
            .host(&cfg.host)
            .port(cfg.port)
            .username(&cfg.username)
            .password(&cfg.password)
            .database(&cfg.database)
    }
}

fn connect_database_with(cfg: DatabaseConfig) -> PgPool {
    PgPool::connect_lazy_with(cfg.into())
}
