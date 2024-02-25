use crate::{queries, types};
use axum::debug_handler;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::{Pool, Postgres};

#[debug_handler]
pub async fn post_transaction(
    State(pool): State<Pool<Postgres>>,
    Path(user_id): Path<i64>,
    Json(transaction): Json<types::Transaction>,
) -> impl IntoResponse {
    let current_balance = queries::get_current_balance(user_id, &pool).await;
    let new_balance = current_balance.balance - transaction.value;
    let limit = -current_balance.limit;

    if new_balance < limit {
        return (StatusCode::UNPROCESSABLE_ENTITY, format!(""));
    }

    let mut db_transaction = pool.begin().await.unwrap();
    queries::insert_transaction(user_id, transaction, &mut *db_transaction).await;
    queries::set_new_balance(user_id, new_balance, &mut *db_transaction).await;
    db_transaction.commit().await.unwrap();
    let current_balance = queries::get_current_balance(user_id, &pool).await;
    let response = types::TransactionResponse {
        balance: current_balance.balance,
        limit: current_balance.limit,
    };
    assert_eq!(new_balance, current_balance.balance);
    let json_string = serde_json::to_string(&response).unwrap();
    (StatusCode::OK, json_string)
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
