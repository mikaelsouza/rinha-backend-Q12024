use crate::types;
struct Statement {}
impl Statement {
    pub async fn get_current_balance(
        user_id: i64,
        tx: &mut sqlx::Transaction<'_, sqlx::postgres::Postgres>,
    ) -> Result<types::Balance, sqlx::Error> {
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
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn get_previous_transactions(
        user_id: i64,
        num_of_results: i64,
        tx: &mut sqlx::Transaction<'_, sqlx::postgres::Postgres>,
    ) -> Result<Vec<types::Transaction>, sqlx::Error> {
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
        .fetch_all(&mut **tx)
        .await
    }
}

pub async fn get_balance_and_transactions(
    user_id: i64,
    num_of_results: i64,
    executor: &sqlx::Pool<sqlx::postgres::Postgres>,
) -> Result<(types::Balance, Vec<types::Transaction>), types::Error> {
    let mut tx = executor.begin().await.unwrap();
    let cur_balance = match Statement::get_current_balance(user_id, &mut tx).await {
        Ok(balance) => balance,
        Err(sqlx::Error::RowNotFound) => return Err(types::Error::UserNotFound),
        Err(_) => panic!(),
    };
    let transactions = Statement::get_previous_transactions(user_id, num_of_results, &mut tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    Ok((cur_balance, transactions))
}

struct Transaction {}
impl Transaction {
    async fn user_exists(
        user_id: i64,
        tx: &mut sqlx::Transaction<'_, sqlx::postgres::Postgres>,
    ) -> Result<(), types::Error> {
        let r = sqlx::query!(
            r#"SELECT balance, NOW() as "timestamp!", "limit" FROM accounts WHERE id = $1"#,
            user_id
        )
        .fetch_one(&mut **tx)
        .await;

        if let Err(sqlx::Error::RowNotFound) = r {
            return Err(types::Error::UserNotFound);
        }
        r.unwrap();
        Ok(())
    }

    async fn lock_accounts(tx: &mut sqlx::Transaction<'_, sqlx::postgres::Postgres>) {
        sqlx::query!(
            r#"
        LOCK TABLE accounts IN ROW EXCLUSIVE MODE
        "#
        )
        .execute(&mut **tx)
        .await
        .unwrap();
    }
    async fn push_transaction(
        user_id: i64,
        transaction: types::Transaction,
        tx: &mut sqlx::Transaction<'_, sqlx::postgres::Postgres>,
    ) -> Result<types::Balance, types::Error> {
        let balance = sqlx::query_as!(types::Balance,
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
        .fetch_one(&mut **tx)
        .await;
        match balance {
            Ok(b) => return Ok(b),
            Err(sqlx::Error::RowNotFound) => Err(types::Error::InconsistentResult),
            Err(_) => panic!(),
        }
    }
}
pub async fn add_transaction(
    user_id: i64,
    transaction: types::Transaction,
    executor: &sqlx::Pool<sqlx::postgres::Postgres>,
) -> Result<types::Balance, types::Error> {
    let mut tx = executor.begin().await.unwrap();
    Transaction::lock_accounts(&mut tx).await;
    match Transaction::user_exists(user_id, &mut tx).await {
        Err(x) => return Err(x),
        Ok(_) => {}
    };
    let balance = Transaction::push_transaction(user_id, transaction, &mut tx).await;
    balance
}
