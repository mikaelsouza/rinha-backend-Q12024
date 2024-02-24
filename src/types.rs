use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::Type;
use time::OffsetDateTime;
#[derive(Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "transaction_type", rename_all = "lowercase")]
pub enum TransactionType {
    C,
    D,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Transaction {
    #[serde(rename = "valor")]
    value: i64,
    #[serde(rename = "tipo")]
    transaction_type: TransactionType,
    #[serde(rename = "descricao")]
    description: String,
    #[serde(default = "OffsetDateTime::now_utc")]
    timestamp: OffsetDateTime,
}
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Balance {
    #[serde(rename = "total")]
    total: i64,
    #[serde(rename = "data_extrato")]
    timestamp: OffsetDateTime, // # TODO: Change this to a date format
    #[serde(rename = "limite")]
    limit: i64,
}
#[derive(Serialize, Deserialize)]
pub struct BalanceResponse {
    #[serde(rename = "saldo")]
    balance: Balance,
    #[serde(rename = "ultimas_transacoes")]
    previous_transactions: Vec<Transaction>,
}
