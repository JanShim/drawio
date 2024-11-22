use std::str::FromStr;

use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_memo, use_state, AttrValue, Callback, Html, InputEvent, MouseEvent, Properties};

use crate::components::shared::{use_state_with, EditButtons};



#[derive(Properties, PartialEq, Debug)]
pub struct Props<T: PartialEq> {
    pub edit_mode: bool,
    pub label: AttrValue,
    pub value: T,
    pub on_commit: Callback<T>,
}

#[function_component]
pub fn PropTableTr<T>(Props::<T> {edit_mode,label,value, on_commit }: &Props<T> ) -> Html 
where T: PartialEq + Clone + ToString + FromStr + 'static
{
    let is_edit = use_state(|| false);
    let original_value = use_memo(value.clone(), |v| v.clone());
    let value_state = use_state_with(value.clone());

    // ============ events ================
    let on_edit = {
        let is_edit = is_edit.clone();
        Callback::from(move |_: MouseEvent| { is_edit.set(true); })
    };  

    let on_apply = {
        let is_edit = is_edit.clone();
        let value = value_state.clone();
        let on_commit = on_commit.clone();
        Callback::from(move |_: MouseEvent| {
            on_commit.emit((*value).clone());
            is_edit.set(false);     // togle is_edit
        })
    };  

    let on_cancel = {
        let is_edit = is_edit.clone();
        let value = value_state.clone();
        Callback::from(move |_: MouseEvent| {
            value.set((*original_value).clone());
            is_edit.set(false);     // togle is_edit
        })
    };      

    let on_value_input = {
        let value = value_state.clone();
        Callback::from(move |e:InputEvent| {
            e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                .map(|input| {
                    if let Ok(new_value) = input.value().parse::<T>() {
                        value.set(new_value);
                    }
                });
        })
    };

    // ============= view ==========
    let value_view = {
        let value = value_state.clone();
        let is_edit = is_edit.clone();
        html! {
            if *edit_mode && *is_edit {
                <input type="number" oninput={on_value_input} value={ (*value).to_string() } step="1"/>
            } else {
                { (*value).to_string() }
            }
        }
    };   


    html! {
        <tr>
        <td class="label" width="20">{ label }</td>
        <td>{ value_view }</td>
        <td class="img">
            <EditButtons {edit_mode}
                {is_edit}
                {on_apply}
                {on_edit}
                {on_cancel}
            />
        </td>
       </tr>
    }
}