use async_trait::async_trait;
use shaku::Interface;

use crate::entity::boardgame::Boardgame;

#[async_trait]
pub trait BoardgameRepository: Interface {
    async fn save(&self, entity: Boardgame);
}
