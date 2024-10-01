use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yewdux::use_dispatch;
use stylist::yew::styled_component;
use web_sys::HtmlDivElement;

use crate::{
    components::{cell_details::CellDetailsComponent, get_global_css}, 
    model::mx_cell::MxCell, 
    store::cell
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub cell: MxCell,
}

#[styled_component(CellComponent)]
pub fn app(Props { cell }: &Props) -> Html {
    let dispatch = use_dispatch::<cell::CellState>();
    let cell = cell.clone();
    let meta = cell.get_meta().unwrap_or_default();
    // This runs only once, on the first render of the component.
    use_effect_with(
        (), // empty deps
        move |_| {
            log::debug!("CellComponent {meta:#?}");
            dispatch.set( cell::CellState { cell, meta });
            || {}
        },
    );    


    // === view items ====
    html! {
    <>
        { get_global_css() }
        <CellDetailsComponent/>
    </>        
    }    
}


#[wasm_bindgen(js_name=renderCell)]
pub fn render_cell(div: HtmlDivElement, cell: MxCell) {
    yew::Renderer::<CellComponent>::with_root_and_props(div.into(), Props {cell}).render();
}
