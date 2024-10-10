use std::sync::Arc;

use crate::database::PgConnectionPoolInterface;
use crate::orms::boardgames;
use async_trait::async_trait;
use domain::{entity::boardgame::Boardgame, repository::boardgame::BoardgameRepository};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, Set};
use shaku::Component;

#[derive(Component)]
#[shaku(interface = BoardgameRepository)]
pub struct BoardgameRepositoryImpl {
    #[shaku(inject)]
    db: Arc<dyn PgConnectionPoolInterface>,
}

#[async_trait]
impl BoardgameRepository for BoardgameRepositoryImpl {
    async fn save(&self, boardgame: Boardgame) {
        let db = self.db.get_connection();

        let model = boardgames::ActiveModel {
            id: NotSet,
            name: Set(boardgame.title),
            thumbnail_url: Set(boardgame.thumbnail_url.map(|v| v.into())),
            record_created_at: NotSet,
            record_updated_at: NotSet,
        };
        model.insert(&db).await.unwrap();
    }
}
