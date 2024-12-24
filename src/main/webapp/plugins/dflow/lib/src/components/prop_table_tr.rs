use std::str::FromStr;

use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_memo, use_state, AttrValue, Callback, Html, InputEvent, MouseEvent, Properties};

use crate::components::shared::{use_state_with, EditButtons};

use super::shared::InputType;



#[derive(Properties, PartialEq, Debug)]
pub struct Props<T: PartialEq> {
    pub name: AttrValue,      // this is <input> element name
    pub label: AttrValue,
    pub value: T,
    pub value_type: InputType,
    pub edit_mode: bool,
    pub checked: bool,
}

#[function_component]
pub fn PropTableTr<T>(Props::<T> { name, label, value, value_type, edit_mode, checked }: &Props<T> ) -> Html
where T: PartialEq + Clone + ToString + FromStr + 'static
{
     // ============= view ==========
    let value_view = {
        let value_html = match value_type {
                InputType::STRING => html! {
                    <input id={ name } name={ name }
                        class="input-100"
                        disabled={ !*checked }
                        value={ value.to_string() }
                    />
                },
                InputType::NUMBER => html! {
                    <input type="number" step="1"
                        id={ name } name={ name }
                        class="input-100"
                        disabled={ !*checked }
                        value={ value.to_string() }
                    />
                },
            };

        html! {
            if *edit_mode {
                { value_html }
            } else {
                { value.to_string() }
            }
        }
    };

    html! {
        <tr>
            <td class="label">{ label }</td>
            <td><div class="prop-value">{ value_view }</div></td>
       </tr>
    }
}