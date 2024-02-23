use axum::debug_handler;
use axum::extract::{Json, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Transaction {
    valor: u128,
    tipo: String, // TODO: Use an enum in the future
    descricao: String,
}

#[debug_handler]
pub async fn post_transaction(
    Path(user_id): Path<u32>,
    Json(transaction): Json<Transaction>,
) -> impl IntoResponse {
    log::info!("User ID: {}", user_id);
    log::info!(
        "Transaction values: {}, {}, {}",
        transaction.valor,
        transaction.tipo,
        transaction.descricao
    );

    (
        StatusCode::OK,
        format!(
            "Info received: {}, {}, {}, {}\n",
            user_id, transaction.valor, transaction.tipo, transaction.descricao,
        ),
    )
}
