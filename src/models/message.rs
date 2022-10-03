pub enum Action {
    // Http,
    Log,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct Message {
    pub action: Action,
    pub payload: String,
    pub action_extra: String,
}
