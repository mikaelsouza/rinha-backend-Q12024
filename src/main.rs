use sqlx::postgres::PgPoolOptions;

mod handlers;
mod setup;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://postgres:my-secret-password@localhost:5432/rinha")
        .await
        .unwrap();

    sqlx::query(r#"INSERT INTO "table1" ("val1", "val2") VALUES ('1', '3')"#)
        .execute(&pool)
        .await
        .unwrap();

    let row: (i32, i32, i32) = sqlx::query_as("SELECT * FROM table1")
        .fetch_one(&pool)
        .await
        .unwrap();

    log::info!("{:?}", row);
    let config = setup::init_environment();

    let routes = setup::init_routes();
    let address = setup::init_address(config.http_port).await;

    axum::serve(address, routes).await.unwrap();
}
