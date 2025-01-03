use common_model::data_source::DataSourceDto;
use serde::{Deserialize, Serialize};
use yew::AttrValue;

use crate::utils::NULL_UUID;

pub mod form;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct WidgetDto {
    pub uuid: String,
    pub name: String,
    pub name_ru: String,
    pub model: String,
    pub group: String,
    pub types: Vec<String>,
    pub geom: Option<String>,
    pub svg: Option<String>,
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
// #[serde(rename_all = "camelCase")]
pub struct WidgetListItem {
    pub uuid: AttrValue,
    pub group: AttrValue,
    pub name: AttrValue,
    pub name_ru: Option<AttrValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
pub struct WidgetGlyphItem {
    pub uuid: AttrValue,
    pub group: AttrValue,
    pub name: AttrValue,
    pub name_ru: Option<AttrValue>,
    pub glyph: AttrValue,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct WidgetProperty {
    pub name: AttrValue,
    pub ds: DataSourceDto,
}