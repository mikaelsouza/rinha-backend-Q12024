mod handlers;
mod queries;
mod setup;
mod types;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let config = setup::init_environment();
    let database = setup::setup_db(&config.db_password).await;
    let routes = setup::init_routes(database);
    let address = setup::init_address(config.http_port).await;
    axum::serve(address, routes).await.unwrap();
}
