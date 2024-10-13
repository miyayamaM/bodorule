use shaku::module;

module! {
    pub AppModule {
        components = [
            infra::repository::health::HealthCheckRepositoryImpl,
            infra::repository::board_game::BoardGameRepositoryImpl,

            // database
            infra::database::PgConnectionPool
        ],
        providers = []
    }
}
