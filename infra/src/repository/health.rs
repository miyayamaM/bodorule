use std::sync::Arc;

use async_trait::async_trait;
use domain::repository::health::HealthCheckRepository;
use sea_orm::{
    sea_query::{self, Query},
    ConnectionTrait, Statement,
};
use shaku::Component;

use crate::database::PgConnectionPoolInterface;

#[derive(Component)]
#[shaku(interface = HealthCheckRepository)]
pub struct HealthCheckRepositoryImpl {
    #[shaku(inject)]
    db: Arc<dyn PgConnectionPoolInterface>,
}

#[async_trait]
impl HealthCheckRepository for HealthCheckRepositoryImpl {
    async fn check_db(&self) -> bool {
        let db = self.db.get_connection();
        let query = Query::select().expr(sea_query::Expr::val(1)).to_owned();

        // クエリを実行
        let builder = db.get_database_backend();
        let stmt: Statement = builder.build(&query);
        db.execute(stmt).await.is_ok()
    }
}
