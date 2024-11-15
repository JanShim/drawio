use std::rc::Rc;
use yew::{function_component, hook, html, AttrValue, Html, Properties};
use yew::use_memo;
use common_model::utils::{map_to_svg_style, map_to_svg_text_style, mx_style_to_map};


#[derive(PartialEq, Debug, Clone)]
pub enum MdIconType {
    Edit,
    Check,
    Back,
    Square,
    Cancel,
    Add,
}

impl MdIconType {
    pub fn get_title(&self) -> AttrValue {
        match self {
            MdIconType::Edit => "Редактировать".into(),
            MdIconType::Check => "Применить".into(),
            MdIconType::Back => "Обратно".into(),
            MdIconType::Square => "-?-".into(),
            MdIconType::Cancel => "Отменить".into(),
            MdIconType::Add => "Добавить".into(),
        }
    }
}

impl Default for MdIconType {
    fn default() -> Self {
        MdIconType::Square
    }
}

impl Into<AttrValue> for MdIconType {
    fn into(self) -> AttrValue {
        match self {
            MdIconType::Edit => "edit_square".into(),
            MdIconType::Check => "check".into(),
            MdIconType::Back => "arrow_back".into(),
            MdIconType::Square => "square".into(),
            MdIconType::Cancel => "cancel".into(),
            MdIconType::Add => "add".into(),
        }
    }
}


#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub icon: MdIconType,
}

#[function_component]
pub fn MdIcon(Props { icon }: &Props) -> Html 
{
    html! {
        <span class="material-icons md-18"  title={icon.get_title()}>{ Into::<AttrValue>::into((*icon).clone()) }</span>
    }
}

#[hook]
pub fn use_css_styles(mx_style: AttrValue) -> Rc<(AttrValue, AttrValue)> {
    use_memo(mx_style, |style| {
        let map = mx_style_to_map(style);
        let style = map_to_svg_style(&map);
        let text_style = map_to_svg_text_style(&map);
        (AttrValue::from(style.to_string()), AttrValue::from(text_style.to_string()))
    })
}