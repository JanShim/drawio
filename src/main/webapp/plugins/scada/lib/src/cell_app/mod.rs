use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yewdux::{use_selector, Dispatch};
use stylist::yew::styled_component;
use web_sys::HtmlDivElement;

use crate::{
    components::{cell_details::{CellDetailsComponent, CellTypeSelectorComponent}, get_global_css}, 
    model::{mx_cell::MxCell, mx_editor::MxEditor, mx_utils::MxUtils}, 
    store, 
    utils::SchemaOptions
};


#[styled_component]
pub fn CellComponent() -> Html {
    let cell_types_num = use_selector(|st: &store::cell::State| {st.meta.types.len()});

    // === view items ====
    html! {
    <>
        { get_global_css() }

        if *cell_types_num > 0 {
            <CellDetailsComponent/>
        } else {
            <CellTypeSelectorComponent/>
        }

    </>        
    }    
}


#[wasm_bindgen(js_name=renderCell)]
pub fn render_cell(mx_editor: MxEditor, mx_utils: MxUtils, cell: MxCell, div: HtmlDivElement, options: SchemaOptions) {
    let meta = cell.get_meta().unwrap_or_default();

    Dispatch::<store::cell::State>::global().set(store::cell::State {
        api_url:options.api_url.unwrap_or("undefiend".to_owned()).into(), 
        cell, 
        meta,
        mx_editor,
        mx_utils,
        ..Default::default()
    });

    yew::Renderer::<CellComponent>::with_root(div.into()).render();
}
