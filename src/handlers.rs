use crate::types::BalanceResponse;
use crate::{queries, types};
use axum::debug_handler;
use axum::extract::{Json, Path, State};
use sqlx::{Pool, Postgres};

#[debug_handler]
pub async fn post_transaction(
    State(state): State<Pool<Postgres>>,
    Path(user_id): Path<i64>,
    Json(transaction): Json<types::Transaction>,
) -> String {
    sqlx::query(queries::INSERT_TRANSACTION)
        .bind(user_id)
        .bind(&transaction.value)
        .bind(&transaction.transaction_type)
        .bind(&transaction.description)
        .execute(&state)
        .await
        .unwrap();
    // TODO: Finish the logic for this handler
    serde_json::to_string(&transaction).unwrap()
}

#[debug_handler]
pub async fn get_balance(State(state): State<Pool<Postgres>>, Path(user_id): Path<i32>) -> String {
    let current_balance = sqlx::query_as(queries::GET_CURRENT_BALANCE)
        .bind(user_id)
        .fetch_one(&state);
    let previous_transactions = sqlx::query_as(queries::GET_PREVIOUS_TRANSACTIONS)
        .bind(user_id)
        .fetch_all(&state);
    let current_balance: types::Balance = current_balance.await.unwrap();
    let previous_transactions: Vec<types::Transaction> = previous_transactions.await.unwrap();
    let balance_response = BalanceResponse {
        balance: current_balance,
        previous_transactions,
    };
    let response = serde_json::to_string(&balance_response).unwrap();
    response
}
