use std::sync::Arc;

use salvo::{handler, Depot};

use crate::domain::DomainModule;

#[handler]
pub fn get_environment(depot: &mut Depot) -> String {
    let domain = depot.obtain::<Arc<DomainModule>>().unwrap();

    return domain.user_module.user_service.get_environment();
}
