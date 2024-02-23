use envconfig::Envconfig;
use std::net::{Ipv4Addr, SocketAddrV4};

use crate::handlers;
use axum::routing::{get, post};
use axum::Router;
use tokio::net::TcpListener;
#[derive(Envconfig, Debug)]
pub struct Config {
    #[envconfig(from = "HTTP_PORT", default = "8080")]
    pub http_port: u16,
}

pub fn init_environment() -> Config {
    Config::init_from_env().unwrap()
}

pub fn init_routes() -> Router {
    Router::new()
        .route("/", get(|| async { String::from("Hello World!") }))
        .route(
            "/clientes/:user_id/transacoes",
            post(handlers::post_transaction),
        )
}

pub async fn init_address(port: u16) -> TcpListener {
    let ip = Ipv4Addr::new(0, 0, 0, 0);

    let ip_port = format!("{:?}:{:?}", ip, port);
    log::info!("Binding server to: {}", ip_port);

    let socket = SocketAddrV4::new(ip, port);
    TcpListener::bind(socket).await.unwrap()
}
