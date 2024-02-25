use crate::types;
use sqlx::postgres::PgExecutor;

pub async fn get_current_balance(user_id: i64, executor: impl PgExecutor<'_>) -> types::Balance {
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
    .fetch_one(executor)
    .await
    .unwrap()
}

pub async fn get_previous_transactions(
    user_id: i64,
    num_of_results: i64,
    executor: impl PgExecutor<'_>,
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
        LIMIT $2
        "#,
        user_id,
        num_of_results
    )
    .fetch_all(executor)
    .await
    .unwrap()
}

pub async fn insert_transaction(
    user_id: i64,
    transaction: types::Transaction,
    executor: impl PgExecutor<'_>,
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
    .execute(executor)
    .await
    .unwrap();
}

pub async fn set_new_balance(user_id: i64, new_balance: i64, executor: impl PgExecutor<'_>) {
    sqlx::query!(
        r#"
        UPDATE accounts
        SET balance = $1
        WHERE id = $2
        "#,
        new_balance,
        user_id
    )
    .execute(executor)
    .await
    .unwrap();
}
