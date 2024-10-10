use shaku::module;

module! {
    pub AppModule {
        components = [
            infra::repository::health::HealthCheckRepositoryImpl,
            infra::repository::boardgame::BoardgameRepositoryImpl,

            // database
            infra::database::PgConnectionPool
        ],
        providers = []
    }
}
