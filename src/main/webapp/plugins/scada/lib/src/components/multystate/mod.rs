use std::{rc::Rc, usize};

use yew::{function_component, html, use_state, Callback, Html, MouseEvent};
use yewdux::use_store;

use data_source::DataSourceComponent;
use state::MultystateStateComponent;

use crate::{
    model::cell_meta::multystate::state::StateMeta, store::cell
};

pub mod state;
pub mod data_source;


#[function_component(MultystateComponent)]
pub fn component() -> Html {
    let (cell_state, cell_state_dispatch) = use_store::<cell::State>();

    let selected_state = use_state(|| {
        let value: Option<Rc<StateMeta>> = None;
        value
    });

    let empty = html!{<div>{"not multystate"}</div>};

    let data_source = {
            if let Some(_) = cell_state.get_multystate().ok()  {
                html! {<DataSourceComponent/>}
            } else {
                empty.clone()
            }
        };  

    let state_select_callback = {
            let selected = selected_state.clone();
            Callback::from(move |meta: Option<Rc<StateMeta>>| {
                // apply changes to multystate meta
                log::debug!("state_select_callback: -> {meta:?}");

                // change selected
                selected.set(meta);
            })
        };
    
    let on_cell_meta_apply: Callback<MouseEvent> = cell_state_dispatch.reduce_callback(|state| { 
            state.apply_meta_to_cell();
            state 
        });


    let on_state_add: Callback<MouseEvent> = cell_state_dispatch.reduce_mut_callback(|state| {
            if let Some(m) = state.get_mut_multystate().ok() {
                let pk = m.states.len().to_string();
                log::debug!("on_add {pk}");
                m.states.push(StateMeta {
                    pk,
                    ..Default::default()
                });    
            };
        });

    // apply changes to multystate meta
    let state_apply_callback: Callback<Rc<StateMeta>> = cell_state_dispatch.reduce_mut_callback_with(|state, meta: Rc<StateMeta>| {
            log::debug!("state_apply_callback: -> {meta:?}");

            if let Some(i) = meta.get_index()  {
                if let Some(multy) = &mut state.meta.multystate {
                    let state = &mut multy.states[i];
                    state.style = meta.style.clone();
                }
            }
        });        


    // ------------ View Items
    let states = {
        let selected = selected_state.clone();        
        if let Some(ms) = cell_state.get_multystate().ok()  {
            ms.states.iter().enumerate()
                .map(|(id, meta)| {
                    let props = state::Props {
                        selected: if let Some(selected) = (*selected).clone() {
                                selected.get_index().unwrap_or(usize::MAX) == id
                            } else {
                                false
                            },
                        select_callback: state_select_callback.clone(), 
                        apply_callback: state_apply_callback.clone(),
                        meta: Rc::new((*meta).clone()),
                    };
                    html! { <MultystateStateComponent ..props/> }
                })
                .collect::<Html>()                
        } else {
            empty
        }
    };    
    
    html! {
        <>
        <div class="flex-box-2">
           <button onclick={on_cell_meta_apply}><img src="images/checkmark.gif"/></button>
        </div>        
        <hr/>
        { data_source }
        <div class="flex-box">{"Состояния"}<button onclick={on_state_add}>{"+"}</button></div>
        { states }
        </>
    }

    
}