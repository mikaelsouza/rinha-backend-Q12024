use crate::{queries, types};
use axum::debug_handler;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::{Pool, Postgres};

#[debug_handler]
pub async fn post_transaction(
    State(state): State<Pool<Postgres>>,
    Path(user_id): Path<u32>,
    Json(transaction): Json<types::Transaction>,
) -> impl IntoResponse {
    log::info!("{:?}", transaction);
    let response = String::from("Hey!");
    (StatusCode::OK, response)
}

#[debug_handler]
pub async fn get_balance(State(state): State<Pool<Postgres>>, Path(user_id): Path<i32>) -> String {
    // Logic for solving the problem
    let current_balance = sqlx::query_as(queries::CURRENT_BALANCE)
        .bind(user_id)
        .fetch_one(&state);
    let previous_transactions = sqlx::query_as(queries::PREVIOUS_TRANSACTIONS)
        .bind(user_id)
        .fetch_all(&state);
    let current_balance: types::Balance = current_balance.await.unwrap();
    let previous_transactions: Vec<types::Transaction> = previous_transactions.await.unwrap();

    // Generating final response

    let response = format!("{:?}, {:?}", current_balance, previous_transactions);
    response
}
