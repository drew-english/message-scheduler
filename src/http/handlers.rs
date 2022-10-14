use axum::{
    extract::{Extension, Json},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Pool, Postgres};
use tokio::{sync::mpsc::UnboundedSender, task};
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
    Extension(msg_delivery_tx): Extension<UnboundedSender<Option<DateTime<Utc>>>>,
) -> StatusCode {
    match crate::models::message::Message::new(
        body.delivery_time.unwrap_or_else(Utc::now),
        body.action.clone(),
        body.payload.clone(),
    )
    .create(&db_pool)
    .await
    {
        Ok(res) => {
            let new_delivery_time = res.delivery_time;
            task::spawn(async move {
                if msg_delivery_tx.send(Some(new_delivery_time)).is_err() {
                    error!("Failed to send new delivery time to process loop");
                }
            });
            StatusCode::CREATED
        }
        Err(err) => {
            let message_info = serde_json::to_string(&body)
                .unwrap_or_else(|_| "[serilization_failure]".to_string());
            error!(
                message_info,
                error = err.to_string(),
                "Failed to create message"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
