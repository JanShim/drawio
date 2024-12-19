use std::rc::Rc;

use common_model::dflow_cell::CellType;
use implicit_clone::unsync::IArray;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yew_hooks::use_unmount;
use stylist::yew::styled_component;
use web_sys::HtmlDivElement;

use crate::{
    components::{cell_details::{CellDetails, CellTypeSelector}, get_global_css},
    model::{ cell_meta::{TypesItem, CELL_TYPE_GEOM, CELL_TYPE_LABEL, CELL_TYPE_MULTY}, mx_cell::MxCell, mx_editor::MxEditor, mx_utils::MxUtils},
    store::cell::CellInfoContext,
    utils::SchemaOptions
};

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct CellInfoComponentProps {
    pub context: CellInfoContext,
}

#[styled_component]
pub fn CellInfoComponent(CellInfoComponentProps { context }: &CellInfoComponentProps) -> Html
{
    use_unmount(|| log::debug!("CellInfoComponent unmount"));

    match context.mx_cell.get_meta() {
        Ok(meta) => {
            let cell_types = meta.types.iter().map(|o| o.get_cell_type()).collect::<Vec<_>>();
            log::debug!("CellComponent run  {cell_types:?}");
            html! {<>
                { get_global_css() }

                <ContextProvider<CellInfoContext> context={context.clone()}>
                    <CellDetails />
                    // if cell_types.contains(&CellType::UNDEFIEND) {
                    //     <CellTypeSelector />
                    // } else {
                    //     <CellDetails />
                    // }
                </ContextProvider<CellInfoContext>>
            </>}
        },
        Err(err) => html! {
            { format!("{err:?}") }
        },
    }
}

#[wasm_bindgen(js_name=renderCellInfo)]
pub fn render_cell_info(mx_cell: MxCell, mx_editor: MxEditor, mx_utils: MxUtils, div: HtmlDivElement, options: SchemaOptions)
{
    // Dispatch::<store::cell::State>::global().set(store::cell::State {
    //     // cell: Some(Rc::new(cell)),
    //     meta,
    //     ..Default::default()
    // });

    let props = CellInfoComponentProps {
            context: CellInfoContext {
                api_url: options.api_url.unwrap_or("undefiend".to_owned()).into(),
                mx_utils,
                mx_editor,
                mx_cell,
                available_types: IArray::<TypesItem>::Rc(Rc::new([
                    TypesItem {
                        name:CELL_TYPE_LABEL.into(),
                        label:"Значение".into(),
                        selected:false,
                        cell_type: CellType::LABEL,
                    },
                    TypesItem {
                        name:CELL_TYPE_MULTY.into(),
                        label:"Множество состояний".into(),
                        selected:false,
                        cell_type: CellType::MULTYSTATE
                    },
                    TypesItem {
                        name:CELL_TYPE_GEOM.into(),
                        label:"Геометрия".into(),
                        selected:false,
                        cell_type: CellType::GEOM
                    },
                ]))
            }.into()
        };

    yew::Renderer::<CellInfoComponent>::with_root_and_props(div.into(), props).render();
    log::info!("cell loaded");
}
