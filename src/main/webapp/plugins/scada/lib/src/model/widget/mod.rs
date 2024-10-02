use implicit_clone::unsync::IString;
use serde::{Deserialize, Serialize};

use crate::utils::NULL_UUID;

pub mod meta;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct WidgetDto {
    pub uuid: String,
    pub name: String,
    pub model: String,
    pub group: String,
    pub types: Vec<String>,
}

impl WidgetDto {
    pub fn new(group: String, name: String, model: String, types: Vec<String>) -> Self {
        WidgetDto {
            uuid: NULL_UUID.to_owned(),
            group,
            name,
            model,
            types,
        }
    }
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
pub struct WidgetListItem {
    pub uuid: IString,
    pub group: IString,
    pub name: IString,
}