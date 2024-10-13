use axum::{
    extract::{self, State},
    Json,
};
use common::error::AppError;
use domain::repository::boardgame::BoardgameRepository;
use std::sync::Arc;
use uuid::Uuid;

use registry::AppModule;
use shaku::HasComponent;

use super::BoardGameResponse;

pub async fn show_board_games(
    State(registry): State<Arc<AppModule>>,
    extract::Path(board_game_id): extract::Path<Uuid>,
) -> Result<Json<BoardGameResponse>, AppError> {
    let boardgame_repository: &dyn BoardgameRepository = registry.resolve_ref();
    let board_game = boardgame_repository.find_by_id(board_game_id).await?;
    Ok(Json(
        board_game
            .ok_or(AppError::EntityNotFoundError("Item Not found".to_string()))?
            .into(),
    ))
}
