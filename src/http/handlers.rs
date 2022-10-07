use axum::{
    extract::{Extension, Json},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Pool, Postgres};
use tracing::error;

#[derive(Deserialize, Serialize)]
pub struct CreateMessage {
    #[serde(rename(serialize = "deliveryTime", deserialize = "deliveryTime"))]
    delivery_time: Option<DateTime<Utc>>,
    action: Value,
    payload: String,
}

pub async fn create_message(
    Json(body): Json<CreateMessage>,
    Extension(db_pool): Extension<Pool<Postgres>>,
) -> StatusCode {
    match crate::models::message::Message::new(
        body.delivery_time.unwrap_or(chrono::prelude::Utc::now()),
        body.action.clone(),
        body.payload.clone(),
    )
    .create(&db_pool)
    .await
    {
        Ok(_) => StatusCode::CREATED,
        Err(err) => {
            let message_info =
                serde_json::to_string(&body).unwrap_or("[serilization_failure]".to_string());
            error!(
                message_info,
                error = err.to_string(),
                "Failed to create message"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
