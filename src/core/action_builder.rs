use std::fmt;

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;

use crate::models::{Message, actions::LogV1};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum ActionType {
    Log,
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
    fn init(attributes: Value, payload: String) -> Result<Self, serde_json::Error>
    where
        Self: std::marker::Sized;
    async fn exec(&self);
}

pub fn parse_action(msg: &Message) -> Result<impl Action, serde_json::Error> {
    let parsed: ParsedAction = serde_json::from_value(msg.action.clone())?;
    let args = (parsed.attributes, msg.payload.clone());
    select_action(parsed.action_type, parsed.version)(args.0, args.1)
}

fn select_action(
    action_type: ActionType,
    version: u16,
) -> fn(Value, String) -> Result<LogV1, serde_json::Error> {
    match (action_type, version) {
        (ActionType::Log, 1) => LogV1::init,
        (ActionType::Log, _) => LogV1::init,
    }
}
