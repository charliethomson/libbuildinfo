use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, valuable::Valuable)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "$type",
    content = "context"
)]
pub enum AgentError {
    #[serde(rename = "dev.thmsn.build_info.extract.agent.unimplemented")]
    #[error(
        "There currently arent any errors that the agent extractor can have, so this is here to make the derives happy :^)"
    )]
    Unimplemented,
}
