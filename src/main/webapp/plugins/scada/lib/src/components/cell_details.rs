use quick_xml::se::to_string;
use yew::{function_component, html, use_state, virtual_dom::VNode, Callback, Html, MouseEvent, Properties};
use yewdux::{use_selector, use_store};

use crate::{components::{
        multystate::{self, MultystateComponent}, 
        value::{self, ValueComponent}
    }, 
    model::cell_meta::value::{ApplyValueMetaAction, ValueMeta}, 
    store::cell
};


#[function_component(CellDetailsComponent)]
pub fn component() -> Html {
    let (cell_state, cell_state_dispatch) = use_store::<cell::CellState>();
    let cell_meta = use_selector(|cell_state: &cell::CellState| cell_state.meta.clone());

    let edit_mode = use_state(|| false);

    let value_apply = cell_state_dispatch.apply_callback(|value: ValueMeta| ApplyValueMetaAction(value));

    let edit_mode_toggle = {
            let edit_mode = edit_mode.clone();
            Callback::from(move |_: MouseEvent| { edit_mode.set(true); })
        };

    let cell_details_apply: Callback<MouseEvent> = {
        let edit_mode = edit_mode.clone();
        let cell_meta = cell_meta.clone();
        Callback::from(move |_: MouseEvent| {
            // log::debug!("cell_details_apply:: {:?}", *cell_meta);
            if let Some(cell) = &cell_state.cell {
                let meta = cell.set_meta(&cell_meta).ok();
                log::debug!("NEW CELL META:: {:?}", meta);
                edit_mode.set(false);
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
            let edit_mode = edit_mode.clone();
            if let Some(value) = multy  {
                let props = yew::props! { multystate::Props { edit_mode: *edit_mode} };
                html!{ <MultystateComponent ..props/> }    
            } else {
                html!{<div/>}
            }
        };

    html! {
        <div>
            <pre width="300">{ to_string(&cell_meta).unwrap()}</pre>
            <div class="flex-box-2" style="background-color: green;">
                if *edit_mode {
                    <button onclick={cell_details_apply}><img src="images/checkmark.gif" width="16" height="16"/></button>
                } else {
                    <button onclick={edit_mode_toggle}><img src="images/edit16.png"/></button>
                }
            </div>            
            // <button onclick={on_add}>{"+"}</button><br/>
            // <ul>{entries}</ul>
            // <div>
            //     <button {onclick}>{"set label"}</button><br/>
            //     <label for="label">{"label: "}</label><input id="label" {oninput}/><br/>
            //     <p>{"label: "}{&*name}</p>
            // </div>                
            // { value_view }
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