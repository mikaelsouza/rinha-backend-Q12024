#[debug_handler]
pub async fn post_transaction(
    State(state): State<Pool<Postgres>>,
    Path(user_id): Path<i64>,
    Json(transaction): Json<types::Transaction>,
) -> String {
    let current_balance: types::Balance = sqlx::query_as(queries::GET_CURRENT_BALANCE)
        .bind(user_id)
        .fetch_one(&state)
        .await
        .unwrap();

    let new_balance = current_balance.balance - transaction.value;
    let limit = -current_balance.limit;
    if new_balance < limit {
        return String::from("Sheesh");
    }
    let mut db_transaction = state.begin().await.unwrap();
    sqlx::query(queries::INSERT_TRANSACTION)
        .bind(user_id)
        .bind(transaction.value)
        .bind(transaction.transaction_type)
        .bind(transaction.description)
        .execute(&mut *db_transaction)
        //.execute(&state)
        .await
        .unwrap();
    sqlx::query(queries::SET_NEW_BALANCE)
        .bind(new_balance)
        .bind(user_id)
        .execute(&mut *db_transaction)
        //.execute(&state)
        .await
        .unwrap();
    db_transaction.commit().await.unwrap();
    let current_balance: types::Balance = sqlx::query_as(queries::GET_CURRENT_BALANCE)
        .bind(user_id)
        .fetch_one(&state)
        //.fetch_one(&mut *db_transaction)
        .await
        .unwrap();
    //db_transaction.commit().await.unwrap();

    assert_eq!(new_balance, current_balance.balance);

    String::from("Tudo show de bola!")
}

#[debug_handler]
pub async fn get_balance(State(state): State<Pool<Postgres>>, Path(user_id): Path<i64>) -> String {
    let current_balance = sqlx::query_as(queries::GET_CURRENT_BALANCE)
        .bind(user_id)
        .fetch_one(&state)
        .await
        .unwrap();
    let previous_transactions = sqlx::query_as(queries::GET_PREVIOUS_TRANSACTIONS)
        .bind(user_id)
        .fetch_all(&state)
        .await
        .unwrap();

    // TODO: Verify which one is more idiomatic
    // let current_balance = queries::get_current_balance(user_id, state).await;
    // let previous_transactions = queries::get_previous_transactions(user_id, state)
    let balance_response = types::BalanceResponse {
        balance: current_balance,
        previous_transactions,
    };
    let response = serde_json::to_string(&balance_response).unwrap();
    response
}
