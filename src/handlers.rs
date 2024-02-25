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
    // Check if the user exists
    let result_balance = queries::get_current_balance(user_id, &pool).await;
    let current_balance: types::Balance;
    match result_balance {
        Ok(balance) => current_balance = balance,
        Err(sqlx::Error::RowNotFound) => return (StatusCode::NOT_FOUND, String::from("")),
        Err(_) => panic!(),
    }

    // Calculate new balance
    let new_balance = current_balance.balance - transaction.value;
    let limit = -current_balance.limit;
    if new_balance < limit {
        return (StatusCode::UNPROCESSABLE_ENTITY, format!(""));
    }

    // Uddate balance
    let mut db_transaction = pool.begin().await.unwrap();
    queries::insert_transaction(user_id, transaction, &mut *db_transaction)
        .await
        .unwrap();
    queries::set_new_balance(user_id, new_balance, &mut *db_transaction)
        .await
        .unwrap();
    db_transaction.commit().await.unwrap();

    // Check if everything is ok. Is this really necessary?
    let current_balance = queries::get_current_balance(user_id, &pool).await.unwrap();
    assert_eq!(new_balance, current_balance.balance);

    // Build response
    let response = types::TransactionResponse {
        balance: current_balance.balance,
        limit: current_balance.limit,
    };
    let json_string = serde_json::to_string(&response).unwrap();
    (StatusCode::OK, json_string)
}

#[debug_handler]
pub async fn get_balance(State(pool): State<Pool<Postgres>>, Path(user_id): Path<i64>) -> String {
    let current_balance = queries::get_current_balance(user_id, &pool).await.unwrap();
    let previous_transactions = queries::get_previous_transactions(user_id, 10, &pool)
        .await
        .unwrap();
    let balance_response = types::BalanceResponse {
        balance: current_balance,
        previous_transactions,
    };
    serde_json::to_string(&balance_response).unwrap()
}
