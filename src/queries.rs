pub const CURRENT_BALANCE: &str = r#"
    SELECT
        "limit"    AS "limit",
        NOW()      AS "timestamp",
        balance    AS "balance"
    FROM accounts
    WHERE id = $1
"#;
pub const PREVIOUS_TRANSACTIONS: &str = r#"
    SELECT
        value               AS "value",
        transaction_type    AS "transaction_type",
        description         AS "description",
        timestamp           AS "timestamp"
    FROM transactions
    WHERE account_id = $1
    ORDER BY timestamp DESC
"#;
