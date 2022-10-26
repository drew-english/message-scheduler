use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{postgres::PgQueryResult, query, FromRow, Pool, Postgres};
use uuid::Uuid;

#[derive(Clone, Deserialize, Debug, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[repr(i16)]
pub enum ActionType {
    Log = 0,
    Http = 1,
    Sns = 2
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Serialize for ActionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ActionType::Log => serializer.serialize_str("log"),
            ActionType::Http => serializer.serialize_str("http"),
            ActionType::Sns => serializer.serialize_str("sns"),
        }
    }
}

#[derive(Clone, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    #[serde(default = "Utc::now")]
    pub delivery_time: DateTime<Utc>,
    pub action_type: ActionType,
    pub version: i16,
    pub attributes: Value,
}

impl Message {
    pub async fn create(&self, db_pool: &Pool<Postgres>) -> Result<&Self, sqlx::Error> {
        query!(
            "INSERT INTO messages (id, delivery_time, action_type, version, attributes) values ($1, $2, $3, $4, $5)",
            self.id,
            self.delivery_time,
            self.action_type as i16,
            self.version,
            self.attributes
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
