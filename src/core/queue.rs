use crate::models::message::{Action, Message};

// pub fn front(limit: Option<i32>) {

// }

pub fn all() -> Vec<Box<Message>> {
    vec![
        Box::new(Message{
            action: Action::Log,
            action_extra: "info".to_string(),
            payload: "this is info".to_string(),
        }),
        Box::new(Message{
            action: Action::Log,
            action_extra: "warn".to_string(),
            payload: "this is warning".to_string(),
        }),
    ]
}
