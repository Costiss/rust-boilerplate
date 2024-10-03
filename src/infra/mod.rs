use std::sync::Arc;

use config::ConfigModule;

pub mod config;

#[derive(Clone)]
pub struct InfraModule {
    pub config: Arc<ConfigModule>,
}

impl InfraModule {
    pub fn new() -> Arc<Self> {
        let config = Arc::new(ConfigModule::new());
        Arc::new(InfraModule { config })
    }
}
