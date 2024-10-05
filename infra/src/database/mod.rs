use shaku::{Component, Interface};
use sqlx::PgPool;

pub mod config;

pub trait PgConnectionPoolInterface: Interface {
    fn get_connection(&self) -> PgPool;
}

#[derive(Component)]
#[shaku(interface = PgConnectionPoolInterface)]
pub struct PgConnectionPool {
    pub pool: PgPool,
}

impl PgConnectionPoolInterface for PgConnectionPool {
    fn get_connection(&self) -> PgPool {
        self.pool.clone()
    }
}
