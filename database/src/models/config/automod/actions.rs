use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "key")]
pub enum Action {
    DirectMessage,
    IncreaseBucket(String),
    DeleteMessage,
    SendLogs,
    Timeout(Timeout),
    Kick,
    Ban
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Timeout {
    /// Duration in secs
    pub duration: i64
}