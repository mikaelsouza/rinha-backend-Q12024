use axum;
use {env_logger, log};
mod setup;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("Starting the server! :)");
    let routes = setup::setup_routes();
    let address = setup::setup_address().await;

    axum::serve(address, routes).await.unwrap();
}
