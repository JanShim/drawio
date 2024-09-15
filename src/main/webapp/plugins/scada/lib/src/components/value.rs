use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_reducer, use_state, Callback, Html, InputEvent, MouseEvent, Properties};

use crate::model::cell_meta::value::{ValueAction, ValueMeta};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub value: ValueMeta,
    #[prop_or_default]
    pub apply: Callback<ValueMeta>,
}

#[function_component(ValueComponent)]
pub fn component(Props {value, apply}: &Props ) -> Html {
    let value_state = use_reducer(|| value.clone());

    let is_edit = use_state(|| false);
    let togle_edit = {
        let edit = is_edit.clone();
        Callback::from(move |_: MouseEvent| { edit.set(!*edit); })
    };  

    let togle_apply = {
        let is_edit = is_edit.clone();
        let value = value_state.clone();
        let apply = apply.clone();
        Callback::from(move |_: MouseEvent| {
            apply.emit((*value).clone());
            is_edit.set(!*is_edit);     // togle is_edit
        })
    };        

    // tag name input
    let oninput = {
            let value = value_state.clone();
            Callback::from(move |e:InputEvent| {
                e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                    .map(|input| {
                        value.dispatch(ValueAction::SetTag(input.value().into()));
                    });
            })
        };

    // item view
    let img_view = {
        let is_edit = is_edit.clone();
        if *is_edit { 
           html! { <img src="images/checkmark.gif" onclick={togle_apply}/> }
        } else {
           html! { <img src="images/edit16.png" onclick={togle_edit}/> }
        }
    };    

    let tag_view = {
        let value = value_state.clone();
        html! {
            if *(is_edit.clone()) {
                <input id="tag" {oninput} value={ format!("{}", value.tag) }/>
            } else {
                {value.tag.clone()}
            }
        }
    };    

    html!{
        <table class="prop-table">
        <td class="label" width="20">{"tag"}</td>
        <td>{ tag_view }</td>
        <td class="img">{ img_view }</td>
        </table>
    }
    
}