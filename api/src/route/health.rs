use axum::{routing::get, Router};
use registry::AppModule;

use crate::handler::health::{health_check, health_check_db};

pub fn build_health_check_routes() -> Router<AppModule> {
    let router = Router::new()
        .route("/", get(health_check))
        .route("/db", get(health_check_db));
    Router::new().nest("health", router)
}
