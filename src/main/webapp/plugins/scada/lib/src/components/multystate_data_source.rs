use stylist::css;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{classes, function_component, html, use_state, Callback, Html, InputEvent, MouseEvent, Properties};
use yewdux::{use_selector, use_store};

use crate::store::cell;

#[function_component(DataSourceComponent)]
pub fn component() -> Html {
    let (state, dispatch) = use_store::<cell::State>();
    let is_edit = use_state(|| false);

    let togle_edit = {
        let edit = is_edit.clone();
        Callback::from(move |_: MouseEvent| { edit.set(!*edit); })
    };    

    let img_src = if *(is_edit.clone()) { "images/checkmark.gif" } else { "images/edit16.png" };

    // tag name input
    let oninput = dispatch.reduce_mut_callback_with(|state, e: InputEvent| {
        e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            .map(|input| {
                if let Some(ds) = state.get_mut_multystate_data_source().ok() {
                    ds.tag = input.value();
                } 
            });
    });

    // item view
    let item = match state.get_multystate_data_source().ok() {
        Some(ds) => {
            if *(is_edit.clone()) {
                html! {
                    <div>
                        <label for="tag">{"tag: "}</label><input id="tag" {oninput} value={ds.tag.clone()}/>
                        <img align="right" src={img_src} onclick={togle_edit}/>
                    </div>
                }
            } else {
                html! {<div class={classes!("test-div")}>{ format!("{ds:#?}") }  <img align="right" src={img_src} onclick={togle_edit}/></div>}
            }
        },
        _ => html! {<div>{"data source not found"}</div>}
    }; 

    html!{
        { item }
    }
    
}