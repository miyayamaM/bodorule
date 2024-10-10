use std::sync::Arc;

use async_trait::async_trait;
use domain::{entity::boardgame::Boardgame, repository::boardgame::BoardgameRepository};
use shaku::Component;

use crate::database::PgConnectionPoolInterface;

#[derive(Component)]
#[shaku(interface = BoardgameRepository)]
pub struct BoardgameRepositoryImpl {
    #[shaku(inject)]
    db: Arc<dyn PgConnectionPoolInterface>,
}

#[async_trait]
impl BoardgameRepository for BoardgameRepositoryImpl {
    async fn save(&self, _boardgame: Boardgame) {
        let db = self.db.get_connection();
        sqlx::query("SELECT 1").fetch_one(&db).await.is_ok();
    }
}
