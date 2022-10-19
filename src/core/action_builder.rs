use std::fmt;

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;

use crate::models::{
    actions::{HttpV1, LogV1},
    Message,
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

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum ActionType {
    Log,
    Http,
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize)]
struct ParsedAction {
    #[serde(rename = "type")]
    action_type: ActionType,
    version: u16,
    attributes: Value,
}

#[async_trait]
pub trait Action {
    fn init(attributes: Value, payload: String) -> Result<Self, ActionError> where Self: std::marker::Sized;
    async fn exec(&self) -> Result<(), ActionError>;
}

pub async fn exec_action(msg: &Message) -> Result<(), ActionError> {
    let parsed: ParsedAction = serde_json::from_value(msg.action.clone())?;
    let args = (parsed.attributes, msg.payload.clone());

    match (parsed.action_type, parsed.version) {
        (ActionType::Log, 1) => LogV1::init(args.0, args.1)?.exec().await,
        (ActionType::Log, _) => LogV1::init(args.0, args.1)?.exec().await,
        (ActionType::Http, 1) => HttpV1::init(args.0, args.1)?.exec().await,
        (ActionType::Http, _) => HttpV1::init(args.0, args.1)?.exec().await,
    }
}
