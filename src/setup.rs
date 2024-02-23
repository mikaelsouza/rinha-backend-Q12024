use envconfig::Envconfig;
use std::net::{Ipv4Addr, SocketAddrV4};

use crate::handlers;
use axum::routing::post;
use axum::Router;
use tokio::net::TcpListener;
#[derive(Envconfig, Debug)]
pub struct Config {
    #[envconfig(from = "HTTP_PORT", default = "8080")]
    pub http_port: u16,
}

pub fn init_environment() -> Config {
    let config = Config::init_from_env().unwrap();
    log::info!("Initializing server with configs: {:?}", config);
    config
}

pub fn init_routes() -> Router {
    Router::new().route(
        "/clientes/:user_id/transacoes",
        post(handlers::post_transaction),
    )
}

pub async fn init_address(port: u16) -> TcpListener {
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let ip_port = format!("{:?}:{:?}", ip, port);
    let socket = SocketAddrV4::new(ip, port);
    log::info!("Binding server to: {}", ip_port);
    TcpListener::bind(socket).await.unwrap()
}
