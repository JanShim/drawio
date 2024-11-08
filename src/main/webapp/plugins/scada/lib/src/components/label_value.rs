use std::rc::Rc;
use common_model::free_value::LabelValueXml;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{html, function_component, use_state, Callback, Html, InputEvent, MouseEvent, Properties};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub value: LabelValueXml,
    #[prop_or_default]
    pub apply: Callback<LabelValueXml>,
}

#[function_component]
pub fn LabelValueComponent(Props {value, apply}: &Props ) -> Html {
    let label_state = use_state(|| value.clone());

    let is_edit = use_state(|| false);
    let togle_edit = {
        let edit = is_edit.clone();
        Callback::from(move |_: MouseEvent| { edit.set(!*edit); })
    };  

    let togle_apply = {
        let is_edit = is_edit.clone();
        let label_state = label_state.clone();
        let apply = apply.clone();
        Callback::from(move |_: MouseEvent| {
            apply.emit((*label_state).clone());
            is_edit.set(!*is_edit);     // togle is_edit
        })
    };        

    // tag name input
    let oninput = {
            let label_state = label_state.clone();
            Callback::from(move |e:InputEvent| {
                e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                    .map(|input| {
                        let mut ds = label_state.ds.clone();
                        ds.set_tag(input.value().into());
                        label_state.set(LabelValueXml { ds });
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
        let value = label_state.clone();
        html! {
            if *(is_edit.clone()) {
                <input id="tag" {oninput} value={ format!("{}", value.ds.tag) }/>
            } else {
                {value.ds.tag.clone()}
            }
        }
    };    

    html!{
        <fieldset>
            <legend>{"Настройки значения:"}</legend>
            
            <table class="prop-table">
                <td class="label" width="20">{"tag"}</td>
                <td>{ tag_view }</td>
                <td class="img">{ img_view }</td>
            </table>
        </fieldset>
    }
    
}