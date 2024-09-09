use quick_xml::se::to_string;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent, MouseEvent,};
use yewdux::use_store;

use crate::store::cell;

#[function_component(CellDetailsComponent)]
pub fn component() -> Html {
    let (state, dispatch) = use_store::<cell::State>();

    let entries = state.entries.iter().enumerate()
        .map(|(id, item)| html! { <li>{ format!("{item}")}</li> })
        .collect::<Html>();

    let on_add: Callback<MouseEvent> = {
        dispatch.reduce_mut_callback(move |state| {
            state.entries.push(123);
        })};

    let name = use_state(|| String::new());
    let oninput = {
        let state = state.clone();
        Callback::from({
            let name = name.clone();
            move |e: InputEvent| {
                let input = e.target()
                    .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                if let Some(input) = input {
                    name.set(input.value());
                }
            }
        })};

    let onclick = {
        let name = name.clone();
        dispatch.reduce_mut_callback(move |state| {
            if let Some(mut meta) = state.meta.clone() {
                meta.set_label((*name).clone());
                let m = meta.clone();
                if let Some(mut cell) = state.cell.clone() {
                    cell.set_meta(&m)
                        .map_err(|er| {
                            log::error!("{er:#?}");
                            er
                        })
                        .unwrap();
                }
            }
        })};        

    match &state.meta {
        Some(meta) => html! {
            <div>
                <pre width="300">{ to_string(&meta).unwrap()}</pre>
                <button onclick={on_add}>{"+"}</button><br/>
                <ul>{entries}</ul>
                <div>
                    <button {onclick}>{"set label"}</button><br/>
                    <label for="label">{"label: "}</label><input id="label" {oninput}/><br/>
                    <p>{"label: "}{&*name}</p>
                </div>                
            </div>
        },
        _ => html! {<div></div>},
    }
}