use common_model::data_source::DataSourceXml;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_reducer, use_state, Callback, Html, InputEvent, MouseEvent, Properties};
use yewdux::use_store;

use crate::{
    components::shared::{MdIcon, MdIconType}, model::cell_meta::data_source_reducers::WidgetApplyDsAction, store::cell 
};



#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub ds: DataSourceXml,
    pub edit_mode: bool,
}

#[function_component(DataSourceComponent)]
pub fn component(Props {ds, edit_mode}: &Props ) -> Html {
    let (_, cell_store_dispatch) = use_store::<cell::State>();

    let ds_state = use_state(|| ds.clone());

    let is_edit = use_state(|| false);
    let togle_edit = {
        let is_edit = is_edit.clone();
        Callback::from(move |_: MouseEvent| { is_edit.set(!*is_edit); })
    };  

    // let toggle_close = {
    //     let edit = is_edit.clone();
    //     Callback::from(move |_: MouseEvent| { 
    //         edit.set(false);
    //     })
    // };  

    let togle_apply = {
        let is_edit = is_edit.clone();
        let ds = ds_state.clone();
        Callback::from(move |_: MouseEvent| {
            cell_store_dispatch.apply(WidgetApplyDsAction((*ds).clone()));
            is_edit.set(!*is_edit);     // togle is_edit
        })
    };        

    // tag name input
    let on_tag_input = {
            let ds_state = ds_state.clone();
            Callback::from(move |e:InputEvent| {
                e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                    .map(|input| {
                        let mut value = (*ds_state).clone();
                        value.set_tag(input.value().into());
                        ds_state.set(value);
                    });
            })
        };

    // item view
    let img_view = {
        let is_edit = is_edit.clone();
        if *edit_mode {
            if *is_edit { 
                html! { <button onclick={togle_apply}><MdIcon icon={MdIconType::Check}/></button> }
             } else {
                html! { <button onclick={togle_edit}><MdIcon icon={MdIconType::Edit}/></button> }
             }
        } else {
            html! { <span/> }
        }
    };    

    let tag_view = {
        let ds = ds_state.clone();
        let is_edit = is_edit.clone();
        html! {
            if *edit_mode && *is_edit {
                <input id="tag" oninput={on_tag_input} value={ format!("{}", ds.tag) }/>
            } else {
                {ds.tag.clone()}
            }
        }
    };    

    html!{
        <table class="prop-table">
        <td class="label" width="70">{"Тэг объекта"}</td>
        <td>{ tag_view }</td>
        <td class="img">{ img_view }</td>
        </table>
    }
    
}