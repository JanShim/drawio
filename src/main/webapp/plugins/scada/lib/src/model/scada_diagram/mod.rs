use serde::{Deserialize, Serialize};


pub mod meta;

pub const NULL_UUID: &str = "00000000-0000-0000-0000-000000000000";


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
pub struct ListItem {
    pub uuid: String,
    pub name: String,
}