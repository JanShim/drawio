use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_reducer, use_state, Callback, Html, InputEvent, MouseEvent, Properties};
use yewdux::{use_store, Reducer};

use crate::{
    errors::CellStateError, 
    model::cell_meta::{
            CellMeta,
            multystate::data_source::{DataSourceAction, DataSourceMeta}, 
        }, 
    store::cell,
};


pub struct MultystateApplyDsAction(DataSourceMeta);
impl Reducer<cell::CellState> for MultystateApplyDsAction {
    fn apply(self, state: Rc<cell::CellState>) -> Rc<cell::CellState> {
        let mut multystate = state.meta.multystate.clone()
            .expect(format!("{}", CellStateError::NotMultystate).as_str());

        multystate.data_source = self.0;

        cell::CellState {
            cell: state.cell.clone(),
            meta: CellMeta { 
                    multystate: Some(multystate), 
                    ..state.meta.clone() 
                },
            }
            .into()            
    }
}

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub ds: DataSourceMeta,
    pub edit_mode: bool,
}

#[function_component(DataSourceComponent)]
pub fn component(Props {ds, edit_mode}: &Props ) -> Html {
    let (_, cell_store_dispatch) = use_store::<cell::CellState>();

    let data_source_state = use_reducer(|| ds.clone());

    let is_edit = use_state(|| false);
    let togle_edit = {
        let edit = is_edit.clone();
        Callback::from(move |_: MouseEvent| { edit.set(!*edit); })
    };  

    let togle_apply = {
        let is_edit = is_edit.clone();
        let ds = data_source_state.clone();
        Callback::from(move |_: MouseEvent| {
            cell_store_dispatch.apply(MultystateApplyDsAction((*ds).clone()));
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
        let ds = data_source_state.clone();
        let is_edit = is_edit.clone();
        html! {
            if *edit_mode && *is_edit {
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