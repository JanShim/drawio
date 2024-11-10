use yew::{function_component, html, AttrValue, Html, Properties};


#[derive(PartialEq, Debug, Clone)]
pub enum MdIconType {
    Edit,
    Check,
    Back,
    Square,
    Cancel,
}

impl MdIconType {
    pub fn get_title(&self) -> AttrValue {
        match self {
            MdIconType::Edit => "Редактировать".into(),
            MdIconType::Check => "Применить".into(),
            MdIconType::Back => "Обратно".into(),
            MdIconType::Square => "-?-".into(),
            MdIconType::Cancel => "Отменить".into(),
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

