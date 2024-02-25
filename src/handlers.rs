use crate::{queries, types};
use axum::debug_handler;
use axum::extract::{Json, Path, State};
use sqlx::{Pool, Postgres};

#[debug_handler]
pub async fn post_transaction(
    State(pool): State<Pool<Postgres>>,
    Path(user_id): Path<i64>,
    Json(transaction): Json<types::Transaction>,
) -> String {
    let current_balance = queries::get_current_balance(user_id, &pool).await;
    let new_balance = current_balance.balance - transaction.value;
    let limit = -current_balance.limit;

    if new_balance < limit {
        return String::from("Sheesh");
    }
    queries::insert_transaction(user_id, transaction, &pool).await;
    queries::set_new_balance(user_id, new_balance, &pool).await;
    let current_balance = queries::get_current_balance(user_id, &pool).await;
    assert_eq!(new_balance, current_balance.balance);
    String::from("Tudo show de bola!")
}

#[debug_handler]
pub async fn get_balance(State(pool): State<Pool<Postgres>>, Path(user_id): Path<i64>) -> String {
    let current_balance = queries::get_current_balance(user_id, &pool).await;
    let previous_transactions = queries::get_previous_transactions(user_id, 10, &pool).await;
    let balance_response = types::BalanceResponse {
        balance: current_balance,
        previous_transactions,
    };

    serde_json::to_string(&balance_response).unwrap()
}
