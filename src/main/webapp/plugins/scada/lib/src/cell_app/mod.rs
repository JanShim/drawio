use yew::prelude::*;
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use yewdux::{use_selector, Dispatch};
use stylist::yew::styled_component;
use web_sys::HtmlDivElement;

use crate::{
    components::{cell_details::{CellDetailsComponent, CellTypeSelectorComponent}, get_global_css}, 
    model::{mx_cell::MxCell, mx_editor::MxEditor, mx_utils::MxUtils}, 
    store::{self, mx_context::{MxGraphContext, TMxGraphContext}}, 
    utils::SchemaOptions
};


#[derive(Properties, Clone, PartialEq, Debug)]
pub struct CellComponentProps {
    pub context: TMxGraphContext, 
}

#[styled_component]
pub fn CellComponent(CellComponentProps { context }: &CellComponentProps) -> Html 
{
    let cell_types_num = use_selector(|st: &store::cell::State| {
        if let Some(meta) = &st.meta {
            return meta.types.len();
        }
        // return
        0
    });

    log::debug!("CellComponent run");

    // === view items ====
    html! {<>
        { get_global_css() }

        <ContextProvider<TMxGraphContext> context={context.clone()}>
            if *cell_types_num > 0 {
                <CellDetailsComponent />
            } else {
                <CellTypeSelectorComponent />
            }
        </ContextProvider<TMxGraphContext>>
    </>}    
}


#[wasm_bindgen(js_name=initCellRender)]
pub fn init_cell_render(mx_editor: MxEditor, mx_utils: MxUtils, div: HtmlDivElement, options: SchemaOptions) {
    log::debug!("init cell render");

    let props = CellComponentProps { 
            context: MxGraphContext { 
                api_url: options.api_url.unwrap_or("undefiend".to_owned()).into(),
                mx_utils, 
                mx_editor 
            }.into() 
        };


    yew::Renderer::<CellComponent>::with_root_and_props(div.into(), props).render();
}


#[wasm_bindgen(js_name=renderCell)]
pub fn render_cell(cell: MxCell) {
    let meta = cell.get_meta().unwrap_or_default();
    log::debug!("render_cell: {meta:?}");

    Dispatch::<store::cell::State>::global().set(store::cell::State {
        cell: Some(Rc::new(cell)), 
        meta: Some(meta),
        ..Default::default()
    });
}
