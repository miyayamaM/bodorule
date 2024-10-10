use sea_orm::DatabaseConnection;
use shaku::{Component, Interface};

pub mod config;

pub trait PgConnectionPoolInterface: Interface {
    fn get_connection(&self) -> DatabaseConnection;
}

#[derive(Component)]
#[shaku(interface = PgConnectionPoolInterface)]
pub struct PgConnectionPool {
    pub db_conn: DatabaseConnection,
}

impl PgConnectionPoolInterface for PgConnectionPool {
    fn get_connection(&self) -> DatabaseConnection {
        self.db_conn.clone()
    }
}
