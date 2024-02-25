use crate::types;
use sqlx::{Pool, Postgres};

pub async fn get_current_balance(user_id: i64, pool: &Pool<Postgres>) -> types::Balance {
    sqlx::query_as!(
        types::Balance,
        r#"
        SELECT
            "limit"    AS "limit",
            NOW()      AS "timestamp!",
            balance    AS "balance"
        FROM accounts
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .unwrap()
}

pub async fn get_previous_transactions(
    user_id: i64,
    state: &Pool<Postgres>,
) -> Vec<types::Transaction> {
    sqlx::query_as!(
        types::Transaction,
        r#"
        SELECT
            value               AS "value",
            transaction_type    AS "transaction_type!: types::TransactionType",
            description         AS "description",
            timestamp           AS "timestamp"
        FROM transactions
        WHERE account_id = $1
        ORDER BY timestamp DESC
        "#,
        user_id
    )
    .fetch_all(state)
    .await
    .unwrap()
}

pub async fn insert_transaction(
    user_id: i64,
    transaction: types::Transaction,
    state: &Pool<Postgres>,
) {
    sqlx::query!(
        r#"
        INSERT INTO "transactions"
            ("account_id", "value", "transaction_type", "description", "timestamp")
        VALUES 
            ($1, $2, $3, $4, NOW())
        "#,
        user_id,
        transaction.value,
        transaction.transaction_type as types::TransactionType,
        transaction.description
    )
    .execute(state)
    .await
    .unwrap();
}

pub async fn set_new_balance(user_id: i64, new_balance: i64, state: &Pool<Postgres>) {
    sqlx::query!(
        r#"
        UPDATE accounts
        SET balance = $1
        WHERE id = $2
        "#,
        new_balance,
        user_id
    )
    .execute(state)
    .await
    .unwrap();
}
