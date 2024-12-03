use serde::Deserialize;
use yew::AttrValue;

#[derive(Debug, PartialEq, Deserialize)]
pub struct WidgetGroupDto {
    pub pk: AttrValue,
    pub name: AttrValue,
    pub model: AttrValue,
    pub image: AttrValue,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct WidgetGroupListItemDto {
    pub pk: AttrValue,
    pub name: AttrValue,
}