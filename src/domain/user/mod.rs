use std::sync::Arc;

use user_service::UserService;

use crate::infra::InfraModule;

pub mod user_service;

#[derive(Clone)]
pub struct UserModule {
    pub user_service: Arc<UserService>,
}

impl UserModule {
    pub fn new(infra: Arc<InfraModule>) -> Arc<Self> {
        Arc::new(UserModule {
            user_service: UserService::new(infra),
        })
    }
}
