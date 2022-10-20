use axum::{
    extract::{Extension, Json},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use tokio::{sync::mpsc::UnboundedSender, task};
use tracing::error;

pub async fn create_message(
    Json(msg): Json<crate::models::Message>,
    Extension(db_pool): Extension<Pool<Postgres>>,
    Extension(msg_delivery_tx): Extension<UnboundedSender<Option<DateTime<Utc>>>>,
) -> StatusCode {
    match msg.create(&db_pool).await {
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
            let message_info = serde_json::to_string(&msg)
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
