use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "undefiend")]
pub struct UndefiendMeta {
}

impl Default for UndefiendMeta {
    fn default() -> Self {
        Self { }
    }
}
