use serde::{Deserialize, Serialize};

use crate::utils::NULL_UUID;


pub mod meta;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ScadaDiagramDto {
    pub uuid: String,
    pub name: String,
    pub model: String,
}

impl ScadaDiagramDto {
    pub fn new(name: String, model: String) -> Self {
        ScadaDiagramDto {
            uuid: NULL_UUID.to_owned(),
            name,
            model,
        }
    }
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
pub struct DiagramListItem {
    pub uuid: String,
    pub name: String,
}