use async_trait::async_trait;
use shaku::Interface;
use uuid::Uuid;

use crate::entity::boardgame::Boardgame;

#[async_trait]
pub trait BoardgameRepository: Interface {
    async fn save(&self, entity: Boardgame);
    async fn find_by_id(&self, id: Uuid) -> Option<Boardgame>;
}
