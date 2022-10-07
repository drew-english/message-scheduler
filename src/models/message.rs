use chrono::{DateTime, Utc};
use sqlx::{postgres::PgQueryResult, query, query_as, types::Uuid, FromRow, Pool, Postgres};

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

    pub async fn create(&self, db_pool: &Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error> {
        query!(
            "INSERT INTO messages (id, delivery_time, action, payload) values ($1, $2, $3, $4)",
            self.id,
            self.delivery_time,
            self.action,
            self.payload,
        )
        .execute(db_pool)
        .await
    }

    pub async fn delete(&self, db_pool: &Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error> {
        query!("DELETE FROM messages WHERE id=$1", self.id)
            .execute(db_pool)
            .await
    }

    pub async fn find_by_delivery_time(
        db_pool: &Pool<Postgres>,
        time: &DateTime<Utc>,
        limit: Option<i64>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let limit_by = limit.unwrap_or(20);
        query_as!(
            Self,
            "SELECT * FROM messages WHERE delivery_time <= $1 LIMIT $2",
            time,
            limit_by,
        )
        .fetch_all(db_pool)
        .await
    }
}
