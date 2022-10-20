use crate::models::Message;
use tracing::{error, info};

use super::action_builder::exec_action;

pub async fn dispatch(msg: Message, db_pool: sqlx::Pool<sqlx::Postgres>) {
    match exec_action(&msg).await {
        Ok(_) => (),
        Err(err) => error!(
            id = msg.id.to_string(),
            action = msg.action_type.to_string(),
            error = err.to_string(),
            "[Dispatcher] Error dispatching message"
        ),
    };

    match msg.delete(&db_pool).await {
        Ok(_) => {
            info!(
                id = msg.id.to_string(),
                action = msg.action_type.to_string(),
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
