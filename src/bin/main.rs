use sea_orm::{Database, DatabaseConnection};
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use anyhow::Result;
use api::route::board_game::build_board_game_routes;
use api::route::health::build_health_check_routes;
use axum::Router;
use infra::database::config::DatabaseConfig;
use infra::database::{PgConnectionPool, PgConnectionPoolParameters};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let config = DatabaseConfig {
        protocol: "postgres".to_string(),
        host: "localhost".to_string(),
        port: 5432,
        username: "app".to_string(),
        password: "passwd".to_string(),
        database: "app".to_string(),
    };

    let db_conn: DatabaseConnection = Database::connect(config.to_database_url()).await?;

    let registry = registry::AppModule::builder()
        .with_component_parameters::<PgConnectionPool>(PgConnectionPoolParameters { db_conn })
        .build();

    let app = Router::new()
        .merge(build_health_check_routes())
        .merge(build_board_game_routes())
        .with_state(Arc::new(registry));

    let address = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    let listener = TcpListener::bind(address).await?;
    println!("listening on {}", address);
    Ok(axum::serve(listener, app).await?)
}
