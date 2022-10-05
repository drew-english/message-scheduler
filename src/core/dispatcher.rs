use crate::models::message;
use async_trait::async_trait;
use tracing::{error, info, warn};

#[async_trait]
trait Action {
    async fn exec(&self);
}

struct LogAction {
    log_type: String,
    log_payload: String,
}

#[async_trait]
impl Action for LogAction {
    async fn exec(&self) {
        match self.log_type.as_str() {
            "info" => info!(message = self.log_payload, "[Dispatcher][LogAction]"),
            "warn" => warn!(message = self.log_payload, "[Dispatcher][LogAction]"),
            "error" => error!(message = self.log_payload, "[Dispatcher][LogAction]"),
            _ => error!(
                log_type = self.log_type,
                "[Dispatcher][LogAction] invalid type given"
            ),
        }
    }
}

pub async fn dispatch(msg: Box<message::Message>) {
    let action = match msg.action {
        message::Action::Log => LogAction {
            log_type: msg.action_extra,
            log_payload: msg.payload,
        },
    };
    action.exec().await;
}
