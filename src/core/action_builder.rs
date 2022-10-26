use std::fmt;

use async_trait::async_trait;
use aws_sdk_sns::types::SdkError;
use serde_json::Value;

use crate::models::{
    actions::{HttpV1, LogV1, SnsV1},
    message::ActionType,
    Message,
};

pub enum ActionError {
    ParseError(serde_json::Error),
    ExecError(String),
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

impl From<SdkError<aws_sdk_sns::error::PublishError>> for ActionError {
    fn from(err: SdkError<aws_sdk_sns::error::PublishError>) -> ActionError {
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
pub trait Action: Send + Sync {
    fn init(attributes: Value) -> Result<Self, ActionError>
    where
        Self: std::marker::Sized;
    async fn exec(&self) -> Result<(), ActionError>;
}

pub fn validate_action(
    Message {
        action_type,
        version,
        attributes,
        ..
    }: &Message,
) -> Option<ActionError> {
    let attr = attributes.clone();
    match (action_type, version) {
        (ActionType::Log, 1) => LogV1::init(attr).err(),
        (ActionType::Log, _) => LogV1::init(attr).err(),
        (ActionType::Http, 1) => HttpV1::init(attr).err(),
        (ActionType::Http, _) => HttpV1::init(attr).err(),
        (ActionType::Sns, 1) => SnsV1::init(attr).err(),
        (ActionType::Sns, _) => SnsV1::init(attr).err(),
    }
}

pub fn create_action(
    Message {
        action_type,
        version,
        attributes,
        ..
    }: &Message,
) -> Result<Box<dyn Action>, ActionError> {
    let attr = attributes.clone();
    Ok(match (action_type, version) {
        (ActionType::Log, 1) => Box::new(LogV1::init(attr)?),
        (ActionType::Log, _) => Box::new(LogV1::init(attr)?),
        (ActionType::Http, 1) => Box::new(HttpV1::init(attr)?),
        (ActionType::Http, _) => Box::new(HttpV1::init(attr)?),
        (ActionType::Sns, 1) => Box::new(SnsV1::init(attr)?),
        (ActionType::Sns, _) => Box::new(SnsV1::init(attr)?),
    })
}

pub async fn exec_action(msg: &Message) -> Result<(), ActionError> {
    create_action(msg)?.exec().await
}
