use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Check {
    FlaggedScamLink,
    TextLines(TextLines),
    CapsLock(CapsLock),
    Invites(Invites),
    Regex(Regex)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Invites {
    // codes of allowed invites
    pub allowed_invites: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
// min and max in %
pub struct CapsLock {
    pub min: Option<u8>,
    pub max: Option<u8>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextLines {
    pub line_len: Option<u16>,
    pub min: Option<u16>,
    pub max: Option<u16>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Regex {
    pub is_matching: bool,
    pub regex: String
}