use chrono::{DateTime, Utc};
use sqlx::{postgres::PgQueryResult, query, types::Uuid, FromRow, Pool, Postgres};

#[derive(Clone, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub delivery_time: DateTime<Utc>,
    pub action: serde_json::Value,
    pub payload: String,
}

impl Message {
    pub fn new(delivery_time: DateTime<Utc>, action: serde_json::Value, payload: String) -> Self {
        Message {
            id: Uuid::new_v4(),
            delivery_time,
            action,
            payload,
        }
    }

    pub async fn create(&self, db_pool: &Pool<Postgres>) -> Result<&Self, sqlx::Error> {
        query!(
            "INSERT INTO messages (id, delivery_time, action, payload) values ($1, $2, $3, $4)",
            self.id,
            self.delivery_time,
            self.action,
            self.payload,
        )
        .execute(db_pool)
        .await?;
        Ok(self)
    }

    pub async fn delete(&self, db_pool: &Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error> {
        query!("DELETE FROM messages WHERE id=$1", self.id)
            .execute(db_pool)
            .await
    }
}
