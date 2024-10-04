use async_trait::async_trait;
use shaku::Interface;

#[async_trait]
pub trait HealthCheckRepository: Interface {
    async fn check_db(&self) -> bool;
}
