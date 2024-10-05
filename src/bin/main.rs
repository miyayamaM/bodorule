use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use anyhow::Result;
use api::route::health::build_health_check_routes;
use axum::Router;
use infra::database::config::DatabaseConfig;
use infra::database::{PgConnectionPool, PgConnectionPoolParameters};
use sqlx::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let config = DatabaseConfig {
        host: "localhost".to_string(),
        port: 5432,
        username: "app".to_string(),
        password: "passwd".to_string(),
        database: "app".to_string(),
    };

    let pool = PgPool::connect_lazy_with(config.clone().into());

    let registry = registry::AppModule::builder()
        .with_component_parameters::<PgConnectionPool>(PgConnectionPoolParameters { pool })
        .build();

    let app = Router::new()
        .merge(build_health_check_routes())
        .with_state(Arc::new(registry));

    let address = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    let listener = TcpListener::bind(address).await?;
    println!("listening on {}", address);
    Ok(axum::serve(listener, app).await?)
}
