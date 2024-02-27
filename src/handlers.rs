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
    let add_transaction = queries::add_transaction(user_id, transaction, &pool).await;
    let current_balance = match add_transaction {
        Ok(balance) => balance,
        Err(types::Error::UserNotFound) => return (StatusCode::NOT_FOUND, String::from("{}")),
        Err(types::Error::InconsistentResult) => {
            return (StatusCode::UNPROCESSABLE_ENTITY, String::from("{}"))
        }
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
    let r = match queries::get_balance_and_transactions(user_id, 10, &pool).await {
        Ok(result) => result,
        Err(types::Error::UserNotFound) => return (StatusCode::NOT_FOUND, String::from("{}")),
        Err(_) => panic!(),
    };
    let balance_response = types::BalanceResponse {
        balance: r.0,
        previous_transactions: r.1,
    };
    (
        StatusCode::OK,
        serde_json::to_string(&balance_response).unwrap(),
    )
}
