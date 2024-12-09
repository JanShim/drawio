use common_model::data_source::DataSourceDto;
use serde::{Deserialize, Serialize};
use yew::AttrValue;

use crate::utils::NULL_UUID;

pub mod form_meta;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct WidgetDto {
    pub uuid: String,
    pub name: String,
    pub model: String,
    pub group: String,
    pub types: Vec<String>,
    pub svg: Option<String>,
}

impl WidgetDto {
    pub fn new(group: String, name: String, model: String, types: Vec<String>, svg: Option<String>) -> Self {
        WidgetDto {
            uuid: NULL_UUID.to_owned(),
            group,
            name,
            model,
            types,
            svg,
        }
    }
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
pub struct WidgetListItem {
    pub uuid: AttrValue,
    pub group: AttrValue,
    pub name: AttrValue,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
pub struct WidgetGlyphItem {
    pub uuid: AttrValue,
    pub group: AttrValue,
    pub name: AttrValue,
    pub glyph: AttrValue,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct WidgetProperty {
    pub name: AttrValue,
    pub ds: DataSourceDto,
}