use axum::{
    extract::{self, State},
    Json,
};
use common::error::AppError;
use domain::repository::board_game::BoardGameRepository;
use std::sync::Arc;
use uuid::Uuid;

use registry::AppModule;
use shaku::HasComponent;

use super::BoardGameResponse;

pub async fn show_board_game(
    State(registry): State<Arc<AppModule>>,
    extract::Path(board_game_id): extract::Path<Uuid>,
) -> Result<Json<BoardGameResponse>, AppError> {
    let board_game_repository: &dyn BoardGameRepository = registry.resolve_ref();
    let board_game = board_game_repository.find_by_id(board_game_id).await?;
    Ok(Json(
        board_game
            .ok_or(AppError::EntityNotFoundError("Item Not found".to_string()))?
            .into(),
    ))
}
