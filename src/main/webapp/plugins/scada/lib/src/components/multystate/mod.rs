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
    let (state, dispatch) = use_store::<cell::State>();

    let selected_state = use_state(|| {
        let value: Option<usize> = None;
        value
    });

    let empty = html!{<div>{"not multystate"}</div>};

    let data_source = {
            if let Some(_) = state.get_multystate().ok()  {
                html! {<DataSourceComponent/>}
            } else {
                empty.clone()
            }
        };  

    let select = {
            let selected = selected_state.clone();
            Callback::from(move |id: Option<usize>| selected.set(id))
        };
    

    let states = {
            let selected = selected_state.clone();        
            if let Some(ms) = state.get_multystate().ok()  {
                ms.states.iter().enumerate()
                    .map(|(id, meta)| {
                        let props = state::Props {
                            selected: (*selected).map(|o| o == id ).unwrap_or(false),
                            select: select.clone(), 
                            meta: (*meta).clone(),
                        };
                        log::debug!("states props: {props:?}");

                        html! { <MultystateStateComponent ..props/> }
                    })
                    .collect::<Html>()                
            } else {
                empty
            }
        };    

    let on_apply: Callback<MouseEvent> = dispatch.reduce_callback(|state| { 
            state.apply_meta_to_cell();
            state 
        });


    let on_add: Callback<MouseEvent> = dispatch.reduce_mut_callback(|state| {
            if let Some(m) = state.get_mut_multystate().ok() {
                let pk = m.states.len().to_string();
                log::debug!("on_add {pk}");
                m.states.push(StateMeta {
                    pk,
                    ..Default::default()
                });    
            };
        });


    // item view
    html! {
        <>
        <div class="flex-box-2">
           <button onclick={on_apply}><img src="images/checkmark.gif"/></button>
        </div>        
        <hr/>
        { data_source }
        <div class="flex-box">{"Состояния"}<button onclick={on_add}>{"+"}</button></div>
        { states }
        </>
    }

    
}