use std::net::{Ipv4Addr, SocketAddrV4};

use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

pub fn setup_routes() -> Router {
    Router::new().route("/", get(|| async { String::from("asd") }))
}

pub async fn setup_address() -> TcpListener {
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let port = 3000;

    let ip_port = format!("{:?}:{:?}", ip, port);

    log::info!("Binding server to: {}", ip_port);

    let socket = SocketAddrV4::new(ip, port);
    TcpListener::bind(socket)
        .await
        .expect(&format!("Failed to bind server to {}", ip_port))
}
