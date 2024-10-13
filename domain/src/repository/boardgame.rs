use async_trait::async_trait;
use common::error::AppError;
use shaku::Interface;
use uuid::Uuid;

use crate::entity::boardgame::Boardgame;

#[async_trait]
pub trait BoardgameRepository: Interface {
    async fn save(&self, entity: Boardgame) -> Result<(), AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Boardgame>, AppError>;
    async fn find_many(&self) -> Result<Vec<Boardgame>, AppError>;
}
