mod handlers;
mod setup;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    log::info!("Starting the server! :)");

    let config = setup::init_environment();
    log::info!("Initializing server with configs: {:?}", config);

    let routes = setup::init_routes();
    let address = setup::init_address(config.http_port).await;

    axum::serve(address, routes).await.unwrap();
}
