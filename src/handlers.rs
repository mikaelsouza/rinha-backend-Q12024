use crate::queries;
use axum::debug_handler;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::{Pool, Postgres, Type};
use time::OffsetDateTime;

#[derive(Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "transaction_type", rename_all = "lowercase")]
enum TransactionType {
    C,
    D,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Transaction {
    value: i64,
    transaction_type: TransactionType,
    description: String,
    timestamp: OffsetDateTime,
}
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Statement {
    balance: i64,
    timestamp: OffsetDateTime, // # TODO: Change this to a date format
    limit: i64,
}

pub struct StatementResponse {
    statement: Statement,
    previous_transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    limite: i64,
    saldo: i64,
}

#[debug_handler]
pub async fn post_transaction(
    State(state): State<Pool<Postgres>>,
    Path(user_id): Path<u32>,
    Json(transaction): Json<Transaction>,
) -> impl IntoResponse {
    let transaction = serde_json::to_string(&transaction).unwrap();
    let response = serde_json::to_string(&TransactionResponse {
        limite: 10000,
        saldo: 100,
    })
    .unwrap();

    log::info!("User ID: {}", user_id);
    log::info!("Transaction values: {}", transaction);
    log::info!("Response: {}", response);
    (StatusCode::OK, response)
}

/*
    TODO:
    How can I reuse the already existing transacion?
    Maybe I should only create a Transacion and insert the
    `realizado_em` field there.
*/

#[debug_handler]
pub async fn get_statement(
    State(state): State<Pool<Postgres>>,
    Path(user_id): Path<i32>,
) -> String {
    let current_statement: Statement = sqlx::query_as(queries::CURRENT_STATEMENT)
        .bind(user_id)
        .fetch_one(&state)
        .await
        .unwrap();

    let previous_transactions: Vec<Transaction> = sqlx::query_as(queries::PREVIOUS_TRANSACTIONS)
        .bind(user_id)
        .fetch_all(&state)
        .await
        .unwrap();

    let response = format!("{:?}, {:?}", current_statement, previous_transactions);
    response
}
