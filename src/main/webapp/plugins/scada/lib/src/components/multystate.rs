use yew::{function_component, html, Callback, Html, MouseEvent};
use yewdux::use_store;

use crate::{
    components::{
        multystate_data_source::DataSourceComponent, 
        multystate_state::MultystateStateComponent
    }, 
    model::cell_meta::{
        // multystate_data_source::DataSource, 
        multystate_state::StateMeta
    }, 
    store::cell
};

#[function_component(MultystateComponent)]
pub fn component() -> Html {
    let (state, dispatch) = use_store::<cell::State>();
    // let meta = use_selector(|state: &cell::State| state.get_ref_meta().ok().unwrap());

    let empty = html!{<div>{"not multystate"}</div>};

    let data_source = {
            if let Some(_) = state.get_multystate().ok()  {
                html! {<DataSourceComponent/>}
            } else {
                empty.clone()
            }
        };  

    let states = {
            if let Some(ms) = state.get_multystate().ok()  {
                ms.states.iter().enumerate()
                    .map(|(id, _)| {
                        html! { <MultystateStateComponent index={id}/> }
                    })
                    .collect::<Html>()                
            } else {
                empty
            }
        };    

    // let states = state.states.iter().enumerate()
    //     .map(|(id, item)| html! { <li>{ format!("{:#?}", item.uuid)}</li> })
    //     .collect::<Html>();

    let on_apply: Callback<MouseEvent> = dispatch.reduce_callback(|state| { 
            state.apply_meta_to_cell();
            state 
        });


    let on_add: Callback<MouseEvent> = dispatch.reduce_mut_callback(|state| {
            if let Some(m) = state.get_mut_multystate().ok() {
                m.states.push(StateMeta {pk: m.states.len().to_string()});    
            };
        });

    // let multystate = use_selector(|state: &cell::State| match state.meta.multystate {
    //     Some(aa) => Default::default(),
    //     _ => Default::default()
    // });


    // if let Some(aaa) =  state.meta  {

    //     return html! {
    //         <div>
    //             {"here multystate"}
    //         </div>
    //     }

    // } else {
    //     return  html! { <div/> }
    // }

    // let multystate = use_selector(|state: &cell::State| 
    //         state.multystate
    //     );

    // let multy: VNode = {
    //         let multy = multystate.clone();
    //         match *multy {
    //             Some(a) => html!{"here"},
    //             _ => html!{ "no multystate" }
    //         }
    //     };

    html! {
        <>
        <div class="flex-box-2">
           <button onclick={on_apply}><img src="images/checkmark.gif"/></button>
        </div>        
        // <div>{"применить"} <button align="right"><img src="images/checkmark.gif"/></button></div>
        <hr/>
        { data_source }
        <div><button align="right" onclick={on_add}>{"+"}</button></div>
        { states }
        </>
    }

    
}