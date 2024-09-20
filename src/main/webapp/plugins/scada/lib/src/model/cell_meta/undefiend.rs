use implicit_clone::sync::IString;
use serde::{Deserialize, Serialize};
use web_sys::FormData;

use super::CellType;

pub fn is_none_undefiend(tst: &Option<UndefiendMeta>) -> bool {
    match tst {
        Some(_) => false,
        None => true,
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "undefiend")]
pub struct UndefiendMeta {
}

impl Default for UndefiendMeta {
    fn default() -> Self {
        Self { }
    }
}
