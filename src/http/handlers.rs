use axum::Extension;
use sqlx::{Pool, Postgres};

pub async fn test(db_pool: Extension<Pool<Postgres>>) -> String {
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&db_pool.0)
        .await
        .unwrap();

    format!("result: {}", row.0)
}
