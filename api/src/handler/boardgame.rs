use axum::{extract::State, http::StatusCode};
use domain::repository::boardgame::BoardgameRepository;
use std::sync::Arc;

use registry::AppModule;
use shaku::HasComponent;

pub async fn create_board_game(State(registry): State<Arc<AppModule>>) -> StatusCode {
    let boardgame_repository: &dyn BoardgameRepository = registry.resolve_ref();
    StatusCode::OK
}
