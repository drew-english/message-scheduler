use std::fmt;

use async_trait::async_trait;
use serde_json::Value;

use crate::models::{
    actions::{HttpV1, LogV1},
    Message, message::ActionType,
};

pub enum ActionError {
    ParseError(serde_json::Error),
    ExecError(String)
}

impl From<serde_json::Error> for ActionError {
    fn from(err: serde_json::Error) -> ActionError {
        ActionError::ParseError(err)
    }
}

impl From<reqwest::Error> for ActionError {
    fn from(err: reqwest::Error) -> ActionError {
        ActionError::ExecError(err.to_string())
    }
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ActionError::ParseError(e) => write!(f, "{}", e),
            ActionError::ExecError(e) => write!(f, "{}", e),
        }
    }
}

#[async_trait]
pub trait Action {
    fn init(attributes: Value) -> Result<Self, ActionError> where Self: std::marker::Sized;
    async fn exec(&self) -> Result<(), ActionError>;
}

pub async fn exec_action(msg: &Message) -> Result<(), ActionError> {
    let attr = msg.attributes.clone();

    match (&msg.action_type, msg.version) {
        (ActionType::Log, 1) => LogV1::init(attr)?.exec().await,
        (ActionType::Log, _) => LogV1::init(attr)?.exec().await,
        (ActionType::Http, 1) => HttpV1::init(attr)?.exec().await,
        (ActionType::Http, _) => HttpV1::init(attr)?.exec().await,
    }
}
