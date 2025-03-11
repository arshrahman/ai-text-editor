use serde::{Deserialize, Serialize};
use std::fmt;


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Command {
    Summarize,
    Paraphrase,
    Expand,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cmd_str = match self {
            Command::Summarize => "summarize",
            Command::Paraphrase => "paraphrase",
            Command::Expand => "expand",
        };
        
        write!(f, "{}", cmd_str)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AskAIRequest {
    pub content: String,
    pub command: Command,
}

#[derive(Debug, Serialize)]
pub struct AskAIResponse {
    pub result: String,
    pub command: String,
}