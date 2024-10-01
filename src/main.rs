use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/health", get(health_check));

    let address = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    let listener = TcpListener::bind(address).await?;
    println!("listening on {}", address);
    Ok(axum::serve(listener, app).await?)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
