use crate::types::{self, Balance};

use sqlx::postgres::{PgExecutor, PgQueryResult};
use sqlx::Error;

pub async fn get_current_balance(
    user_id: i64,
    executor: impl PgExecutor<'_>,
) -> Result<Balance, Error> {
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
}

pub async fn get_previous_transactions(
    user_id: i64,
    num_of_results: i64,
    executor: impl PgExecutor<'_>,
) -> Result<Vec<types::Transaction>, Error> {
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
}

pub async fn insert_transaction(
    user_id: i64,
    transaction: types::Transaction,
    executor: impl PgExecutor<'_>,
) -> Result<PgQueryResult, Error> {
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
}

pub async fn set_new_balance(
    user_id: i64,
    new_balance: i64,
    executor: impl PgExecutor<'_>,
) -> Result<PgQueryResult, Error> {
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
}

pub async fn check_user_exists(user_id: i64, executor: impl PgExecutor<'_>) -> bool {
    let query_result = sqlx::query_as!(
        types::Balance,
        r#"SELECT balance, NOW() as "timestamp!", "limit" FROM accounts WHERE id = $1"#,
        user_id,
    )
    .fetch_one(executor)
    .await;
    match query_result {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

pub async fn add_transaction(
    user_id: i64,
    transaction: types::Transaction,
    executor: impl PgExecutor<'_>,
) -> Result<Balance, Error> {
    sqlx::query_as!(types::Balance,
        r#"
        WITH account_info AS (
            SELECT balance, "limit"
            FROM accounts
            WHERE id = $1
        ),
        new_transaction AS (
            SELECT
                $1::bigint AS account_id,
                $2::bigint AS value,
                $3::transaction_type AS transaction_type,
                $4::text AS description,
                NOW() AS timestamp
            FROM account_info
            WHERE ($3 = 'D' AND account_info.balance - $2 >= -account_info.limit)
               OR ($3 = 'C') -- No need to check limit for credit transactions
        ),
        inserted_transaction AS (
            INSERT INTO "transactions" ("account_id", "value", "transaction_type", "description", "timestamp")
            SELECT * FROM new_transaction
            RETURNING "account_id", "value"
        ),
        update_balance AS (
            UPDATE accounts
            SET balance = CASE
                WHEN $3 = 'D' THEN balance - COALESCE((SELECT "value" FROM inserted_transaction), 0)
                ELSE balance + (SELECT "value" FROM inserted_transaction)
                END
            WHERE EXISTS (SELECT 1 FROM inserted_transaction) AND id = $1
            RETURNING balance, "limit"
        )
        SELECT *, NOW() AS "timestamp!" FROM update_balance;"#,
        user_id,
        transaction.value,
        transaction.transaction_type as types::TransactionType,
        transaction.description
    )
    .fetch_one(executor)
    .await
}
