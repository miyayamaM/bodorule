use async_trait::async_trait;
use domain::repository::health::HealthCheckRepository;
use shaku::Component;
use sqlx::PgPool;

#[derive(Component)]
#[shaku(interface = HealthCheckRepository)]
pub struct HealthCheckRepositoryImpl {
    db: PgPool,
}

#[async_trait]
impl HealthCheckRepository for HealthCheckRepositoryImpl {
    async fn check_db(&self) -> bool {
        sqlx::query("SELECT 1").fetch_one(&self.db).await.is_ok()
    }
}
