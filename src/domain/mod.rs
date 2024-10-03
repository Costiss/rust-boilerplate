use user::UserModule;

use crate::infra::InfraModule;
use std::sync::Arc;

pub mod user;

#[derive(Clone)]
pub struct DomainModule {
    pub user_module: Arc<UserModule>,
}

impl DomainModule {
    pub fn new(infra: Arc<InfraModule>) -> Arc<Self> {
        Arc::new(DomainModule {
            user_module: UserModule::new(infra),
        })
    }
}
