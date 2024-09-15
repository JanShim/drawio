use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_reducer, use_state, Callback, Html, InputEvent, MouseEvent, Properties};
use yewdux::use_store;

use crate::{model::cell_meta::multystate::data_source::{DataSource, DataSourceAction}, store::cell};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub ds: DataSource,
    pub apply: Callback<DataSource>,
}

#[function_component(DataSourceComponent)]
pub fn component(Props {ds, apply}: &Props ) -> Html {
    // let (cell_store, cell_store_dispatch) = use_store::<cell::CellState>();
    let data_source_state = use_reducer(|| (*ds).clone());

    let is_edit = use_state(|| false);
    let togle_edit = {
        let edit = is_edit.clone();
        Callback::from(move |_: MouseEvent| { edit.set(!*edit); })
    };  

    let togle_apply = {
        let is_edit = is_edit.clone();
        let ds = data_source_state.clone();
        let apply = apply.clone();
        Callback::from(move |_: MouseEvent| {
            apply.emit((*ds).clone());
            is_edit.set(!*is_edit);     // togle is_edit
        })
    };        

    // tag name input
    let oninput = {
            let ds = data_source_state.clone();
            Callback::from(move |e:InputEvent| {
                e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                    .map(|input| {
                        ds.dispatch(DataSourceAction::SetTag(input.value().into()));
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
        let ds = data_source_state.clone();
        html! {
            if *(is_edit.clone()) {
                <input id="tag" {oninput} value={ format!("{}", ds.tag) }/>
            } else {
                {ds.tag.clone()}
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