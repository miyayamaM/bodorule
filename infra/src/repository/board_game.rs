use std::sync::Arc;

use crate::orms::board_games;
use crate::{database::PgConnectionPoolInterface, orms};
use async_trait::async_trait;
use common::error::AppError;
use domain::{entity::board_game::BoardGame, repository::board_game::BoardGameRepository};
use sea_orm::EntityTrait;
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, Set};
use shaku::Component;
use uuid::Uuid;

#[derive(Component)]
#[shaku(interface = BoardGameRepository)]
pub struct BoardGameRepositoryImpl {
    #[shaku(inject)]
    db: Arc<dyn PgConnectionPoolInterface>,
}

#[async_trait]
impl BoardGameRepository for BoardGameRepositoryImpl {
    async fn save(&self, board_game: BoardGame) -> Result<(), AppError> {
        let db = self.db.get_connection();

        let model = board_games::ActiveModel {
            id: NotSet,
            name: Set(board_game.name),
            thumbnail_url: Set(board_game.thumbnail_url.map(|v| v.into())),
            record_created_at: NotSet,
            record_updated_at: NotSet,
        };
        model.insert(&db).await?;
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<BoardGame>, AppError> {
        let db = self.db.get_connection();

        let model = orms::board_games::Entity::find_by_id(id).one(&db).await?;

        Ok(model.map(|m| m.try_into()).transpose()?)
    }

    async fn find_many(&self) -> Result<Vec<BoardGame>, AppError> {
        let db = self.db.get_connection();

        let model = orms::board_games::Entity::find().all(&db).await?;

        Ok(model
            .into_iter()
            .map(|m| m.try_into())
            .collect::<Result<Vec<_>, _>>()?)
    }
}
