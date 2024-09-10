use yew::{function_component, html, Callback, Html, MouseEvent};
use yewdux::{use_selector, use_store};

use crate::{components::multystate_state::MultystateStateComponent, model::cell_meta::multystate_state::StateMeta, store::cell};



#[function_component(MultystateComponent)]
pub fn component() -> Html {
    let (state, dispatch) = use_store::<cell::State>();
    // let meta = use_selector(|state: &cell::State| state.get_ref_meta().ok().unwrap());

    let states = {
            if let Some(m) = state.get_ref_meta().ok()  {
                if let Some(ms) = m.get_multystate().ok() {
                    ms.states.iter().enumerate()
                        .map(|(id, _)| {
                            html! { <MultystateStateComponent index={id}/> }
                        })
                        .collect::<Html>()                
                } else {
                    html!{<div>{"NO!!"}</div>}
                }
            } else {
                html!{<div>{"NO!!"}</div>}
            }
        };    

    // let states = state.states.iter().enumerate()
    //     .map(|(id, item)| html! { <li>{ format!("{:#?}", item.uuid)}</li> })
    //     .collect::<Html>();

    let on_add: Callback<MouseEvent> = {
        dispatch.reduce_mut_callback(move |state| {
            if let Some(m) = state.get_mut_multystate().ok() {
                m.states.push(StateMeta {id: m.states.len().to_string()});    
            };
        })};


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
        <button onclick={on_add}>{"+"}</button><br/>
        <ul>
            { states }
            // {"here multystate"}
        </ul>
        </>
    }

    
}