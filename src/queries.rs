pub const GET_CURRENT_BALANCE: &str = r#"
    SELECT
        "limit"    AS "limit",
        NOW()      AS "timestamp",
        balance    AS "balance"
    FROM accounts
    WHERE id = $1
"#;
pub const GET_PREVIOUS_TRANSACTIONS: &str = r#"
    SELECT
        value               AS "value",
        transaction_type    AS "transaction_type",
        description         AS "description",
        timestamp           AS "timestamp"
    FROM transactions
    WHERE account_id = $1
    ORDER BY timestamp DESC
"#;
pub const INSERT_TRANSACTION: &str = r#"
    INSERT INTO "transactions"
    ("account_id", "value", "transaction_type", "description", "timestamp")
    VALUES ($1, $2, $3, $4, NOW())
"#;
