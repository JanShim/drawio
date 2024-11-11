use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yewdux::{use_selector, use_store, Dispatch};
use stylist::yew::styled_component;
use web_sys::HtmlDivElement;

use crate::{
    components::{cell_details::{CellDetailsComponent, CellTypeSelectorComponent}, get_global_css}, 
    model::{mx_cell::MxCell, mx_editor::MxEditor, mx_utils::MxUtils}, 
    store, 
    utils::SchemaOptions
};


#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub cell_id: AttrValue,
}

#[styled_component]
pub fn CellComponent(Props { cell_id }: &Props) -> Html {
    // let state = use_store::<store::cell::State>();

    let cell_types_num = use_selector(|st: &store::cell::State| {
        st.meta.types.len()
    });

    log::debug!("CellComponent run");

    // let aaa = use_memo(deps, f);

    // === view items ====
    html! {
    <>
        { get_global_css() }

        if *cell_types_num > 0 {
            <CellDetailsComponent cell_id={cell_id}/>
        } else {
            <CellTypeSelectorComponent cell_id={cell_id}/>
        }
    </>        
    }    
}


#[wasm_bindgen(js_name=renderCell)]
pub fn render_cell(mx_editor: MxEditor, mx_utils: MxUtils, cell: MxCell, div: HtmlDivElement, options: SchemaOptions) {
    let cell_id = cell.get_id().unwrap();
    let meta = cell.get_meta().unwrap_or_default();
    
    log::debug!("render_cell: {cell_id:?}, {meta:?}");

    Dispatch::<store::cell::State>::global().set(store::cell::State {
        api_url:options.api_url.unwrap_or("undefiend".to_owned()).into(), 
        cell, 
        meta,
        mx_editor,
        mx_utils,
        ..Default::default()
    });

    let props = Props { cell_id: cell_id.into() };

    yew::Renderer::<CellComponent>::with_root_and_props(div.into(), props).render();
}
