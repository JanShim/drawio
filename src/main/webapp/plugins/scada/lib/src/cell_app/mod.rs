use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yew_hooks::use_effect_once;
use std::rc::Rc;
use web_sys::HtmlDivElement;
use yewdux::{use_selector, use_store};

use crate::{components::cell_details::CellDetailsComponent, model::mx_cell::MxCell, store::cell};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub cell: MxCell,
}

#[function_component(CellComponent)]
pub fn app(Props {cell: mxcell}: &Props) -> Html {
    let (_state, dispatch) = use_store::<cell::State>();

    let cell = mxcell.clone();
    let dispatcher = dispatch.clone();
    use_effect_once(move || {
        let mut new_state = cell::State {cell: Some(cell), ..Default::default()};
        new_state.set_meta_from_self();
        
        dispatcher.set(new_state);
        
        move || dispatcher.set(cell::State {..Default::default()})
    });

    // let up = dispatch.reduce_mut_callback(|state| state.inc());    
    // let dwn = dispatch.reduce_mut_callback(|state| state.dec());    

    // let counter = use_state(|| 0);
    // let up = {
    //     let counter = counter.clone();
    //     Callback::from(move |_: MouseEvent| {
    //         counter.set(*counter + 1);
    //     })
    // };
    // let dwn = {
    //     let counter = counter.clone();
    //     Callback::from(move |_: MouseEvent| {
    //         counter.set(*counter - 1);
    //     })
    // };

    // if let Ok(el) = props.cell.get_value() {
    //     // if let Some(style) = props.cell.mx_style() {
    //     //     el.set_attribute("style", style.as_str()).ok();
    //     // }

    //     // let ch = el.children();
    //     // for i in 0..ch.length() {
    //     //     if let Some(e) = ch.item(i) {
    //     //         e.set_attribute("new-attr", "new value").ok();
    //     //         log::info!("cell attributes: {:?}", e.get_attribute_names());
    //     //     }
    //     // }
    // }

    // let up = Callback::from(move |e: Event| {
    //     // let target: EventTarget = e
    //     //     .target()
    //     //     .expect("Event should have a target when dispatched");
    //     // // You must KNOW target is a HtmlInputElement, otherwise
    //     // // the call to value would be Undefined Behaviour (UB).
    //     // // Here we are sure that this is input element so we can convert it to the appropriate type without checking
    //     // input_value_handle.set(target.unchecked_into::<HtmlInputElement>().value());
    //     let counter = counter.clone();
    //     let value = *counter + 1;
    //     counter.set(value);
    // });    

    html! {
        <div>
            // <button onclick={up}>{ "+1" }</button><button onclick={dwn}>{ "-1" }</button>
            // <p>{ state.count }</p>
            <CellDetailsComponent/>
        </div>
    }    
}


#[wasm_bindgen(js_name=renderCell)]
pub fn render_cell(div: HtmlDivElement, cell: MxCell) {
    yew::Renderer::<CellComponent>::with_root_and_props(div.into(), Props {cell}).render();
}
