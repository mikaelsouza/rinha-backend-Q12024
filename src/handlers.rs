use axum::debug_handler;
use axum::extract::{Json, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    valor: u128,
    tipo: String, // TODO: Use an enum in the future
    descricao: String,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    limite: u128,
    saldo: u128,
}

#[debug_handler]
pub async fn post_transaction(
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
