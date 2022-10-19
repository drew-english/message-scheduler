use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;
use tracing::{error, info, warn};

use crate::core::action_builder::{Action, ActionError};

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum LogType {
    Info,
    Warn,
    Error,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogV1 {
    log_type: LogType,
    #[serde(skip)]
    log_msg: String,
}

#[async_trait]
impl Action for LogV1 {
    fn init(attributes: Value, payload: String) -> Result<Self, ActionError> {
        let mut s: Self = serde_json::from_value(attributes)?;
        s.log_msg = payload;
        Ok(s)
    }

    async fn exec(&self) -> Result<(), ActionError> {
        match self.log_type {
            LogType::Info => info!(message = self.log_msg, "[Message][LogAction]"),
            LogType::Warn => warn!(message = self.log_msg, "[Message][LogAction]"),
            LogType::Error => error!(message = self.log_msg, "[Message][LogAction]"),
        };
        Ok(())
    }
}
