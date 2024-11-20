use std::rc::Rc;
use yew::{function_component, hook, html, AttrValue, Callback, Html, MouseEvent, Properties, UseStateHandle};
use yew::use_memo;
use common_model::utils::{map_to_svg_style, mx_style_to_map};
use yew::{Hook, use_state, use_effect_with};


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
        let (style, text_style) = map_to_svg_style(&map);
        (AttrValue::from(style.to_string()), AttrValue::from(text_style.to_string()))
    })
}

#[hook]
pub fn use_state_with<T>(deps: T) -> UseStateHandle<T>
where 
    T: PartialEq + Clone + 'static,
{
    let state = use_state(|| deps.clone());
    {
        let state = state.clone();
        use_effect_with(deps, move |deps| {
            state.set(deps.clone());
        })   
    }

    // result
    state
}


#[hook]
pub fn use_list_selected<T>() -> (UseStateHandle<Option<T>>, Callback<Option<T>> )
where T: 'static
{
    let selected = use_state(|| {
        let value: Option<T> = None;
        value
    });

    let select_callback = {
        let selected = selected.clone();
        Callback::from(move |value: Option<T>| {
            selected.set(value);  // change selected
        })
    };

    // result
    (selected, select_callback)

}




#[derive(Properties, PartialEq, Debug)]
pub struct EditButtonsProps {
    pub edit_mode: bool,
    pub is_edit: UseStateHandle<bool>,
    pub on_apply: Callback<MouseEvent>,
    pub on_cancel: Callback<MouseEvent>,
    pub on_edit: Callback<MouseEvent>
}

#[function_component]
pub fn EditButtons(EditButtonsProps { 
    edit_mode, 
    is_edit, 
    on_apply, 
    on_edit,
    on_cancel,
}: &EditButtonsProps ) -> Html 
{
    if *edit_mode {
        if **is_edit { 
            html! {<div style="width:64px"> 
                <button onclick={on_apply}><MdIcon icon={MdIconType::Check}/></button>
                <button onclick={on_cancel}><MdIcon icon={MdIconType::Cancel}/></button>
            </div>}
         } else {
            html! { <button onclick={on_edit}><MdIcon icon={MdIconType::Edit}/></button> }
         }
    } else {
        html! { <span/> }
    }
}
