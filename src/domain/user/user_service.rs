use std::sync::Arc;

use crate::infra::{config::ConfigModule, InfraModule};

#[derive(Clone)]
pub struct UserService {
    config: Arc<ConfigModule>,
}

impl UserService {
    pub fn new(infra: Arc<InfraModule>) -> Arc<Self> {
        let config = infra.config.clone();

        Arc::new(UserService { config })
    }

    pub fn get_environment(&self) -> String {
        return self.config.environment.clone();
    }
}
