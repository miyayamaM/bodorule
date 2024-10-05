use shaku::module;

module! {
    pub AppModule {
        components = [
            infra::repository::health::HealthCheckRepositoryImpl,

            // database
            infra::database::PgConnectionPool
        ],
        providers = []
    }
}
