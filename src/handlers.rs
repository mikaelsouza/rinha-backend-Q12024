use axum::debug_handler;
use axum::extract::{Json, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    valor: i64,
    tipo: String, // TODO: Use an enum in the future
    descricao: String,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    limite: i64,
    saldo: i64,
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

/*
    TODO:
    How can I reuse the already existing transacion?
    Maybe I should only create a Transacion and insert the
    `realizado_em` field there.
*/

pub struct RecordedTransaction {
    valor: i64,
    tipo: String,
    descricao: String,
    realizado_em: String,
}

pub struct Statement {
    total: i64,
    data_extrato: String, // # TODO: Change this to a date format
    limite: i64,
    ultimas_transacoes: Vec<RecordedTransaction>,
}

impl Statement {
    fn init() -> Self {
        Self {
            total: 0,
            data_extrato: String::from(""),
            limite: 0,
            ultimas_transacoes: Vec::new(),
        }
    }
}

pub async fn get_statement(Path(user_id): Path<u32>) -> impl IntoResponse {
    todo!();
    let s = Statement::init();
    (StatusCode::OK, format!(""))
}
