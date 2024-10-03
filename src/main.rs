use salvo::prelude::*;

pub mod api;
pub mod domain;
pub mod infra;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let infra = infra::InfraModule::new();
    let domain = domain::DomainModule::new(infra);
    let router = api::RestAPIModule::router(domain.clone());

    let router = Router::new().hoop(affix_state::inject(domain)).push(router);
    let acceptor = TcpListener::new("127.0.0.1:3000").bind().await;
    Server::new(acceptor).serve(router).await;
}
