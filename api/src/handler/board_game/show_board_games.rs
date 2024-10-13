use axum::{extract::State, Json};
use common::error::AppError;
use domain::repository::board_game::BoardGameRepository;
use std::sync::Arc;

use registry::AppModule;
use shaku::HasComponent;

use super::BoardGameResponse;

pub async fn show_board_games(
    State(registry): State<Arc<AppModule>>,
) -> Result<Json<Vec<BoardGameResponse>>, AppError> {
    let board_game_repository: &dyn BoardGameRepository = registry.resolve_ref();
    let board_game = board_game_repository.find_many().await?;
    Ok(Json(board_game.into_iter().map(|v| v.into()).collect()))
}
