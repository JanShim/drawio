use implicit_clone::unsync::IString;
use serde::{Deserialize, Serialize};

use crate::utils::NULL_UUID;

pub mod form_meta;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct DiagramDto {
    pub uuid: String,
    pub name: String,
    pub model: String,
    pub svg: Option<String>,
}

impl DiagramDto {
    pub fn new(name: String, model: String, svg: Option<String>) -> Self {
        DiagramDto {
            uuid: NULL_UUID.to_owned(),
            name,
            model,
            svg,
        }
    }
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
pub struct DiagramListItem {
    pub uuid: IString,
    pub name: IString,
}