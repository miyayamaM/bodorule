use async_trait::async_trait;
use common::error::AppError;
use shaku::Interface;
use uuid::Uuid;

use crate::entity::board_game::BoardGame;

#[async_trait]
pub trait BoardGameRepository: Interface {
    async fn save(&self, entity: BoardGame) -> Result<(), AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<BoardGame>, AppError>;
    async fn find_many(&self) -> Result<Vec<BoardGame>, AppError>;
}
