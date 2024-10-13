use axum::{extract::State, Json};
use common::error::AppError;
use domain::repository::boardgame::BoardgameRepository;
use std::sync::Arc;

use registry::AppModule;
use shaku::HasComponent;

use super::BoardGameResponse;

pub async fn show_board_game(
    State(registry): State<Arc<AppModule>>,
) -> Result<Json<Vec<BoardGameResponse>>, AppError> {
    let boardgame_repository: &dyn BoardgameRepository = registry.resolve_ref();
    let board_game = boardgame_repository.find_many().await?;
    Ok(Json(board_game.into_iter().map(|v| v.into()).collect()))
}
