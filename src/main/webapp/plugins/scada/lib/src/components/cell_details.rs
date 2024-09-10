use quick_xml::se::to_string;
use yew::{function_component, html, virtual_dom::VNode, Html};
use yewdux::{use_selector, use_store};

use crate::{components::multystate::MultystateComponent, store::cell};

#[function_component(CellDetailsComponent)]
pub fn component() -> Html {
    // let (state, dispatch) = use_store::<cell::State>();
    let meta = use_selector(|state: &cell::State| state.meta.clone());

    let multystate: VNode = {
            let meta = meta.clone();
            if let Some(_) = (*meta).multystate  {
                html!{ 
                    <MultystateComponent/>
                }    
            } else {
                html!{<div/>}
            }
        };


    // let entries = state.entries.iter().enumerate()
    //     .map(|(id, item)| html! { <li>{ format!("{item}")}</li> })
    //     .collect::<Html>();

    // let on_add: Callback<MouseEvent> = {
    //     dispatch.reduce_mut_callback(move |state| {
    //         state.entries.push(123);
    //     })};

    // let name = use_state(|| String::new());
    // let oninput = {
    //     let state = state.clone();
    //     Callback::from({
    //         let name = name.clone();
    //         move |e: InputEvent| {
    //             let input = e.target()
    //                 .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

    //             if let Some(input) = input {
    //                 name.set(input.value());
    //             }
    //         }
    //     })};

    // let onclick = {
    //     let name = name.clone();
    //     dispatch.reduce_mut_callback(move |state| {
    //         let mut meta = state.meta.clone();
    //         meta.set_label((*name).clone());
    //         // set to cell if exist
    //         if let Some(mut cell) = state.cell.clone() {
    //             cell.set_meta(&meta)
    //                 .map_err(|er| {
    //                     log::error!("{er:#?}");
    //                     er
    //                 })
    //                 .unwrap();
    //         }

    //         // if let Some(mut meta) = state.meta.clone() {
    //         //     meta.set_label((*name).clone());
    //         //     let m = meta.clone();
    //         //     if let Some(mut cell) = state.cell.clone() {
    //         //         cell.set_meta(&m)
    //         //             .map_err(|er| {
    //         //                 log::error!("{er:#?}");
    //         //                 er
    //         //             })
    //         //             .unwrap();
    //         //     }
    //         // }
    //     })};        
    
    html! {
        <div>
            <pre width="300">{ to_string(&meta).unwrap()}</pre>
            // <button onclick={on_add}>{"+"}</button><br/>
            // <ul>{entries}</ul>
            // <div>
            //     <button {onclick}>{"set label"}</button><br/>
            //     <label for="label">{"label: "}</label><input id="label" {oninput}/><br/>
            //     <p>{"label: "}{&*name}</p>
            // </div>                

            { multystate }

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