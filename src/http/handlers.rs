use axum::Extension;
use sqlx::{types::Uuid, Pool, Postgres};

pub async fn test(db_pool: Extension<Pool<Postgres>>) -> &'static str {
    let msg = crate::models::message::Message {
        id: Uuid::new_v4(),
        delivery_time: chrono::prelude::Utc::now(),
        action: serde_json::json!({"version": 1}),
        payload: "tesing1234".to_string(),
    };
    msg.create(&db_pool.0).await.unwrap();

    "done"
}
