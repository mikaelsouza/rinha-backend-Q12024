use crate::{queries, types};
use axum::debug_handler;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::{Pool, Postgres};

// lmao this is terrible, please kill me
#[debug_handler]
pub async fn post_transaction(
    State(pool): State<Pool<Postgres>>,
    Path(user_id): Path<i64>,
    Json(transaction): Json<types::Transaction>,
) -> impl IntoResponse {
    if queries::check_user_exists(user_id, &pool).await == false {
        return (StatusCode::NOT_FOUND, String::from("{}"));
    }

    let add_transaction = queries::add_transaction(user_id, transaction, &pool).await;
    let current_balance = match add_transaction {
        Ok(balance) => balance,
        Err(sqlx::Error::RowNotFound) => {
            return (StatusCode::UNPROCESSABLE_ENTITY, String::from("{}"))
        }
        Err(_) => panic!(),
    };
    let response = types::TransactionResponse {
        balance: current_balance.balance,
        limit: current_balance.limit,
    };
    let json_string = serde_json::to_string(&response).unwrap();
    (StatusCode::OK, json_string)
}

#[debug_handler]
pub async fn get_balance(
    State(pool): State<Pool<Postgres>>,
    Path(user_id): Path<i64>,
) -> impl IntoResponse {
    let current_balance = match queries::get_current_balance(user_id, &pool).await {
        Ok(balance) => balance,
        Err(sqlx::Error::RowNotFound) => return (StatusCode::NOT_FOUND, String::from("{}")),
        Err(_) => panic!(),
    };
    let previous_transactions = queries::get_previous_transactions(user_id, 10, &pool)
        .await
        .unwrap();
    let balance_response = types::BalanceResponse {
        balance: current_balance,
        previous_transactions,
    };
    (
        StatusCode::OK,
        serde_json::to_string(&balance_response).unwrap(),
    )
}
