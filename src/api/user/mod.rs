use salvo::Router;

mod handlers;

pub struct UserAPI {}

impl UserAPI {
    pub fn router() -> Router {
        Router::with_path("/v1/users")
            .push(Router::with_path("/env").get(handlers::get_environment))
    }
}
