use config::DatabaseConfig;
use shaku::{Component, Interface};
use sqlx::PgPool;

pub mod config;

pub trait PgConnectionPoolInterface: Interface {
    fn connect(&self) -> PgPool;
}

#[derive(Component)]
#[shaku(interface = PgConnectionPoolInterface)]
pub struct PgConnectionPool {
    pub config: DatabaseConfig,
}

impl PgConnectionPoolInterface for PgConnectionPool {
    fn connect(&self) -> PgPool {
        PgPool::connect_lazy_with(self.config.clone().into())
    }
}
