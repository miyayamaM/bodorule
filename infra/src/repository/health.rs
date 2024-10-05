use std::sync::Arc;

use async_trait::async_trait;
use domain::repository::health::HealthCheckRepository;
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
        let db = self.db.connect();
        sqlx::query("SELECT 1").fetch_one(&db).await.is_ok()
    }
}
