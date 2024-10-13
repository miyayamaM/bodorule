use std::sync::Arc;

use crate::orms::boardgames;
use crate::{database::PgConnectionPoolInterface, orms};
use async_trait::async_trait;
use common::error::AppError;
use domain::{entity::boardgame::Boardgame, repository::boardgame::BoardgameRepository};
use sea_orm::EntityTrait;
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, Set};
use shaku::Component;
use uuid::Uuid;

#[derive(Component)]
#[shaku(interface = BoardgameRepository)]
pub struct BoardgameRepositoryImpl {
    #[shaku(inject)]
    db: Arc<dyn PgConnectionPoolInterface>,
}

#[async_trait]
impl BoardgameRepository for BoardgameRepositoryImpl {
    async fn save(&self, boardgame: Boardgame) -> Result<(), AppError> {
        let db = self.db.get_connection();

        let model = boardgames::ActiveModel {
            id: NotSet,
            name: Set(boardgame.name),
            thumbnail_url: Set(boardgame.thumbnail_url.map(|v| v.into())),
            record_created_at: NotSet,
            record_updated_at: NotSet,
        };
        model.insert(&db).await?;
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Boardgame>, AppError> {
        let db = self.db.get_connection();

        let model = orms::boardgames::Entity::find_by_id(id).one(&db).await?;

        Ok(model.map(|m| m.try_into()).transpose()?)
    }

    async fn find_many(&self) -> Result<Vec<Boardgame>, AppError> {
        let db = self.db.get_connection();

        let model = orms::boardgames::Entity::find().all(&db).await?;

        Ok(model
            .into_iter()
            .map(|m| m.try_into())
            .collect::<Result<Vec<_>, _>>()?)
    }
}
