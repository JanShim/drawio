use std::rc::Rc;

use quick_xml::se::to_string;
use yew::{function_component, html, virtual_dom::VNode, Callback, Html, MouseEvent};
use yewdux::{use_selector, use_store, Reducer};

use crate::{components::{
        multystate::{self, MultystateComponent}, 
        value::{self, ValueComponent}
    }, 
    model::cell_meta::{
        CellMeta, 
        multystate::MultystateMeta, 
        value::ValueMeta, 
    }, 
    store::cell
};


struct ApplyValueMeta(ValueMeta);
impl Reducer<cell::CellState> for ApplyValueMeta {
    fn apply(self, state: Rc<cell::CellState>) -> Rc<cell::CellState> {
        cell::CellState {
            meta: CellMeta { value: Some(self.0), ..(state.meta.clone()) },
            cell: state.cell.clone(),
        }.into()        
    }
}

struct ApplyMultyStateMeta(MultystateMeta);
impl Reducer<cell::CellState> for ApplyMultyStateMeta {
    fn apply(self, state: Rc<cell::CellState>) -> Rc<cell::CellState> {
        cell::CellState {
            meta: CellMeta { multystate: Some(self.0), ..(state.meta.clone()) },
            cell: state.cell.clone(),
        }.into()        
    }
}

#[function_component(CellDetailsComponent)]
pub fn component() -> Html {
    let (cell_state, cell_state_dispatch) = use_store::<cell::CellState>();
    let cell_meta = use_selector(|cell_state: &cell::CellState| cell_state.meta.clone());

    let value_apply = cell_state_dispatch.apply_callback(|value: ValueMeta| ApplyValueMeta(value));
    let multystate_apply = cell_state_dispatch.apply_callback(|value: MultystateMeta| ApplyMultyStateMeta(value));

    let cell_details_apply: Callback<MouseEvent> = {
        let cell_meta = cell_meta.clone();
        // let cell_state = cell_state.clone();
        // let multy_state = multy_state.clone();
        // cell_store_dispatch.reduce_mut_callback(move |cell_state| {
        //     log::debug!("cell_meta_apply:: multy {:?}", *multy_state);

        //     cell_state.set_multystate((*multy_state).clone());
        //     cell_state.apply_meta_to_cell();
        // }
        Callback::from(move |_: MouseEvent| {
            log::debug!("cell_details_apply:: {:?}", *cell_meta);
            if let Some(cell) = &cell_state.cell {
                let meta = cell.set_meta(&cell_meta).ok();
                log::debug!("NEW CELL META:: {:?}", meta);
            }
        })
    };

    // component views
    let value_view: VNode = {
            let value_meta = cell_meta.value.clone();
            if let Some(value) = value_meta  {
                let props = yew::props! { value::Props { value, apply: value_apply} };
                html!{ <ValueComponent ..props/> }    
            } else {
                html!{<div/>}
            }
        };
    
    let multystate_view: VNode =  {
            let multy = cell_meta.multystate.clone();
            if let Some(value) = multy  {
                let props = yew::props! { multystate::Props { value, apply: multystate_apply} };
                html!{ <MultystateComponent ..props/> }    
            } else {
                html!{<div/>}
            }
        };

    html! {
        <div>
            <pre width="300">{ to_string(&cell_meta).unwrap()}</pre>
            <div class="flex-box-2" style="background-color: green;">
                <button onclick={cell_details_apply}><img src="images/checkmark.gif"/></button>
            </div>            
            // <button onclick={on_add}>{"+"}</button><br/>
            // <ul>{entries}</ul>
            // <div>
            //     <button {onclick}>{"set label"}</button><br/>
            //     <label for="label">{"label: "}</label><input id="label" {oninput}/><br/>
            //     <p>{"label: "}{&*name}</p>
            // </div>                
            { value_view }
            { multystate_view }

        </div>
    }


    // match &state.meta {
    //     Some(meta) => html! {
    //         <div>
    //             <pre width="300">{ to_string(&meta).unwrap()}</pre>
    //             <button onclick={on_add}>{"+"}</button><br/>
    //             <ul>{entries}</ul>
    //             <div>
    //                 <button {onclick}>{"set label"}</button><br/>
    //                 <label for="label">{"label: "}</label><input id="label" {oninput}/><br/>
    //                 <p>{"label: "}{&*name}</p>
    //             </div>                
    //         </div>
    //     },
    //     _ => html! {<div></div>},
    // }
}