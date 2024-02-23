mod handlers;
mod setup;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let config = setup::init_environment();
    let routes = setup::init_routes();
    let address = setup::init_address(config.http_port).await;

    axum::serve(address, routes).await.unwrap();
}
