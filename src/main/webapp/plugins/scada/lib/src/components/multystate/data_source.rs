use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent, MouseEvent, Properties};
use yewdux::{use_store, Reducer};
use common_model::{data_source::DataSourceXml, multystate::MultystateXml};

use crate::{
    model::cell_meta::{CellMeta, CellMetaVariant}, 
    store::cell,
};


pub struct MultystateApplyDsAction(pub DataSourceXml);
impl Reducer<cell::State> for MultystateApplyDsAction {
    fn apply(self, state: Rc<cell::State>) -> Rc<cell::State> {
        // if let CellMetaVariant::Multystate(multy) = &state.meta.data  {
        //     return cell::State {
        //         meta: CellMeta { 
        //             data: CellMetaVariant::Multystate(
        //                 MultystateXml {
        //                     ds: self.0,
        //                     ..multy.clone()
        //                 }
        //             ),
        //             ..state.meta.clone()
        //         },
        //         ..(*state).clone()
        //     }.into()            
        // }

        todo!();

        // multystate.data_source = self.0;
        state
    }
}

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub ds: DataSourceXml,
    pub edit_mode: bool,
}

#[function_component]
pub fn DataSourceComponent(Props {ds, edit_mode}: &Props ) -> Html {
    let (_, cell_store_dispatch) = use_store::<cell::State>();

    let data_source = use_state(|| ds.clone());

    let is_edit = use_state(|| false);
    let togle_edit = {
        let is_edit = is_edit.clone();
        Callback::from(move |_: MouseEvent| { is_edit.set(!*is_edit); })
    };  

    let togle_apply = {
        let is_edit = is_edit.clone();
        let ds = data_source.clone();
        Callback::from(move |_: MouseEvent| {
            cell_store_dispatch.apply(MultystateApplyDsAction((*ds).clone()));
            is_edit.set(!*is_edit);     // togle is_edit
        })
    };        

    // tag name input
    let on_tag_input = {
            let ds = data_source.clone();
            Callback::from(move |e:InputEvent| {
                e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                    .map(|input| {
                        let mut val = (*ds).clone();
                        val.tag = input.value().into();
                        ds.set(val);
                    });
            })
        };

    // item view
    let img_view = {
        let is_edit = is_edit.clone();
        if *edit_mode {
            if *is_edit { 
                html! { <img src="images/checkmark.gif" onclick={togle_apply}/> }
             } else {
                html! { <img src="images/edit16.png" onclick={togle_edit}/> }
             }
        } else {
            html! { <span/> }
        }

    };    

    let tag_view = {
        let ds = data_source.clone();
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
        <td class="label" width="20">{"Тэг свойства"}</td>
        <td>{ tag_view }</td>
        <td class="img">{ img_view }</td>
        </table>
    }
    
}