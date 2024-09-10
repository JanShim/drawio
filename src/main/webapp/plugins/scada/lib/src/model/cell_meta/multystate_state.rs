use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "state")]
pub struct StateMeta {
    #[serde(rename = "@id")]
    pub id: String,
}

impl Default for StateMeta {
    fn default() -> Self {
        Self { id: Default::default() }
    }
}