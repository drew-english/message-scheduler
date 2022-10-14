use crate::models::Message;
use tracing::{error, info};

use super::action_builder::{parse_action, Action};

pub async fn dispatch(msg: Message, db_pool: sqlx::Pool<sqlx::Postgres>) {
    match parse_action(&msg) {
        Ok(action) => action.exec().await,
        Err(err) => error!(
            id = msg.id.to_string(),
            action = msg.action.to_string(),
            error = err.to_string(),
            "[Dispatcher] Error parsing message into action"
        ),
    };

    match msg.delete(&db_pool).await {
        Ok(_) => {
            info!(
                id = msg.id.to_string(),
                action = msg.action.to_string(),
                desired_delivery_time = msg.delivery_time.to_string(),
                "[Dipatcher] Finished dispatching message",
            );
        }
        Err(err) => error!(
            error = err.to_string(),
            "[Dipatcher] Error deleting message"
        ),
    };
}
