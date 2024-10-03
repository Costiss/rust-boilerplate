mod user;

use std::sync::Arc;

use salvo::prelude::*;
use user::UserAPI;

use crate::domain::DomainModule;

pub struct RestAPIModule {}

impl RestAPIModule {
    pub fn router(domain: Arc<DomainModule>) -> Router {
        Router::new()
            .hoop(affix_state::inject(domain.clone()))
            .push(UserAPI::router())
    }
}
