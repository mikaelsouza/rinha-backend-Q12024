use crate::handlers;
use axum::routing::{get, post};
use axum::Router;
use envconfig::Envconfig;
use sqlx::{Pool, Postgres};
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::TcpListener;

#[derive(Envconfig, Debug)]
pub struct Config {
    #[envconfig(from = "HTTP_PORT", default = "8099")]
    pub http_port: u16,
    #[envconfig(from = "DB_PASSWORD", default = "test")]
    pub db_password: String,
}

pub fn init_environment() -> Config {
    let config = Config::init_from_env().unwrap();
    log::info!("Initializing server with configs: {:?}", config);
    config
}

pub fn init_routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route(
            "/clientes/:user_id/transacoes",
            post(handlers::post_transaction),
        )
        .route("/clientes/:user_id/extrato", get(handlers::get_statement))
        .with_state(db)
}

pub async fn init_address(port: u16) -> TcpListener {
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let ip_port = format!("{:?}:{:?}", ip, port);
    let socket = SocketAddrV4::new(ip, port);
    log::info!("Binding server to: {}", ip_port);
    TcpListener::bind(socket).await.unwrap()
}

pub async fn setup_db(db_password: String) -> Pool<Postgres> {
    let url = format!("postgres://postgres:{}@localhost:5432/rinha", db_password);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await
        .unwrap();
    pool
}
