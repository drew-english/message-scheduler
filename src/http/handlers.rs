use axum::{
    extract::{Extension, Json},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};
use tokio::{sync::mpsc::UnboundedSender, task};
use tracing::error;

use crate::core::action_builder::validate_action;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn create_message(
    Json(msg): Json<crate::models::Message>,
    Extension(db_pool): Extension<Pool<Postgres>>,
    Extension(msg_delivery_tx): Extension<UnboundedSender<Option<DateTime<Utc>>>>,
) -> (StatusCode, Json<Value>) {
    let action_err = validate_action(&msg);
    if action_err.is_some() {
        return (
            StatusCode::BAD_REQUEST,
            json!({ "error": action_err.unwrap().to_string() }).into(),
        );
    }

    match msg.create(&db_pool).await {
        Ok(msg) => {
            let new_delivery_time = msg.delivery_time;
            task::spawn(async move {
                if msg_delivery_tx.send(Some(new_delivery_time)).is_err() {
                    error!("Failed to send new delivery time to process loop");
                }
            });
            (StatusCode::CREATED, json!({ "message_id": msg.id }).into())
        }
        Err(err) => {
            let message_info = serde_json::to_string(&msg)
                .unwrap_or_else(|_| "[serilization_failure]".to_string());
            error!(
                message_info,
                error = err.to_string(),
                "Failed to create message"
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "error": "Internal error, failed to create message" }).into(),
            )
        }
    }
}
