use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use domain::repository::health::HealthCheckRepository;
use registry::AppModule;
use shaku::HasComponent;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn health_check_db(State(registry): State<Arc<AppModule>>) -> StatusCode {
    let health_repository: &dyn HealthCheckRepository = registry.resolve_ref();
    if health_repository.check_db().await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

// #[tokio::test]
// async fn test_health_check() {
//     let response = health_check().await;
//     assert_eq!(response, StatusCode::OK);
// }

// #[sqlx::test]
// async fn test_health_check_db(pool: PgPool) {
//     let response = health_check_db(State(pool)).await;
//     assert_eq!(response, StatusCode::OK);
// }
