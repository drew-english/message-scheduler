use crate::models::message::Message;
use chrono::prelude::Utc;
use sqlx::{Pool, Postgres};
use tracing::error;

pub async fn needing_delivery(db_pool: &Pool<Postgres>, limit: Option<i64>) -> Vec<Message> {
    match Message::find_by_delivery_time(db_pool, &Utc::now(), limit).await {
        Ok(messages) => messages,
        Err(err) => {
            error!(error = err.to_string(), "[Queue] Error querying messages");
            Vec::new()
        }
    }
}
