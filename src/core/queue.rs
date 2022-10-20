use crate::models::Message;
use chrono::{DateTime, Utc};
use sqlx::{query, query_as, Pool, Postgres, Row};
use tracing::error;

pub async fn needing_delivery(db_pool: &Pool<Postgres>, limit: Option<i64>) -> Vec<Message> {
    match find_by_delivery_time(db_pool, &Utc::now(), limit).await {
        Ok(messages) => messages,
        Err(err) => {
            error!(error = err.to_string(), "[Queue] Error querying messages");
            Vec::new()
        }
    }
}

pub async fn next_delivery_time(
    db_pool: &Pool<Postgres>,
) -> Result<Option<DateTime<Utc>>, sqlx::Error> {
    let row = query("SELECT delivery_time FROM messages ORDER BY delivery_time ASC LIMIT 1")
        .fetch_optional(db_pool)
        .await?;

    match row {
        Some(row) => Ok(row.try_get("delivery_time")?),
        None => Ok(None),
    }
}

async fn find_by_delivery_time(
    db_pool: &Pool<Postgres>,
    time: &DateTime<Utc>,
    limit: Option<i64>,
) -> Result<Vec<Message>, sqlx::Error> {
    let limit_by = limit.unwrap_or(20);
    query_as!(
        Message,
        "SELECT id, delivery_time, action_type AS \"action_type: _\", version, attributes FROM messages WHERE delivery_time <= $1 LIMIT $2",
        time,
        limit_by,
    )
    .fetch_all(db_pool)
    .await
}
