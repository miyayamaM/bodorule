use shaku::module;

module! {
    pub AppModule {
        components = [infra::repository::health::HealthCheckRepositoryImpl],
        providers = []
    }
}

impl Clone for AppModule {
    fn clone(&self) -> Self {
        AppModule::builder().build()
    }
}
