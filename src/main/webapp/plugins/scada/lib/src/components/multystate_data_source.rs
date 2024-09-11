use stylist::css;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{classes, function_component, html, use_state, Callback, Html, InputEvent, MouseEvent, Properties};
use yewdux::{use_selector, use_store};

use crate::{model::cell_meta::multystate_data_source::DataSource, store::cell};

#[function_component(DataSourceComponent)]
pub fn component() -> Html {
    let (state, dispatch) = use_store::<cell::State>();
    let is_edit = use_state(|| false);
    let data_source = use_state(|| {
        state.get_multystate_data_source().ok()
            .map(|o| (*o).clone())
            .unwrap_or(DataSource::default())
    });

    let togle_edit = {
        let edit = is_edit.clone();
        Callback::from(move |_: MouseEvent| { edit.set(!*edit); })
    };  

    let togle_apply = {
        let is_edit = is_edit.clone();
        let dispatch = dispatch.clone();        
        let ds = data_source.clone();
        Callback::from(move |_: MouseEvent| {
            dispatch.reduce_mut(|state| {
                state.set_multystate_data_source((*ds).clone()).ok();
            });
            is_edit.set(!*is_edit); 
        })
    };        

    let img = {
        let is_edit = is_edit.clone();
        if *is_edit { 
           html! { <img align="right" src="images/checkmark.gif" onclick={togle_apply}/> }
        } else {
           html! { <img align="right" src="images/edit16.png" onclick={togle_edit}/> }
        }
    };

    // tag name input
    let oninput = {
            let ds = data_source.clone();
            Callback::from(move |e:InputEvent| {
                e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                    .map(|input| {
                        let new = DataSource {tag: input.value(), ..(*ds).clone()};
                        ds.set(new);
                    });
            })
        };

    // item view
    let item = {
        let ds = data_source.clone();
        if *(is_edit.clone()) {
            html! {
                <div>
                    <label for="tag">{"tag: "}</label><input id="tag" {oninput} value={ds.tag.clone()}/>
                    { img }
                </div>
            }
        } else {
            html! {<div class={classes!("test-div")}>{ format!("{:#?}", *ds) } { img }</div>}
        }
    };
    

    html!{
        { item }
    }
    
}