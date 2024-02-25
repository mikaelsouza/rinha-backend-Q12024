use serde::{Deserialize, Serialize};
use sqlx::Type;
use time::OffsetDateTime;

#[derive(Type, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "transaction_type")]
pub enum TransactionType {
    C,
    D,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    #[serde(rename = "valor")]
    pub value: i64,
    #[serde(rename = "tipo")]
    pub transaction_type: TransactionType,
    #[serde(rename = "descricao")]
    pub description: String,
    #[serde(
        default = "OffsetDateTime::now_utc",
        rename(serialize = "realizada_em"),
        with = "time::serde::rfc3339"
    )]
    pub timestamp: OffsetDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionResponse {
    #[serde(rename = "valor")]
    pub balance: i64,
    #[serde(rename = "limite")]
    pub limit: i64,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Balance {
    #[serde(rename = "total")]
    pub balance: i64,
    #[serde(rename = "data_extrato", with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    #[serde(rename = "limite")]
    pub limit: i64,
}
#[derive(Serialize, Deserialize)]
pub struct BalanceResponse {
    #[serde(rename = "saldo")]
    pub balance: Balance,
    #[serde(rename = "ultimas_transacoes")]
    pub previous_transactions: Vec<Transaction>,
}
