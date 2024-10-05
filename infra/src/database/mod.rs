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
    #[shaku(default)]
    pub config: DatabaseConfig,
}

impl PgConnectionPoolInterface for PgConnectionPool {
    fn connect(&self) -> PgPool {
        println!("{:#?}", self.config.clone());
        PgPool::connect_lazy_with(self.config.clone().into())
    }
}
