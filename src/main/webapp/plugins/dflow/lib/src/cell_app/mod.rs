use common_model::dflow_cell::CellType;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yew_hooks::use_unmount;
use stylist::yew::styled_component;
use web_sys::HtmlDivElement;

use crate::{
    components::{cell_details::{CellDetails, CellTypeSelector}, get_global_css},
    model::{ mx_cell::MxCell, mx_editor::MxEditor, mx_utils::MxUtils},
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
                    if cell_types.contains(&CellType::UNDEFIEND) {
                        <CellTypeSelector />
                    } else {
                        <CellDetails />
                    }
                </ContextProvider<CellInfoContext>>
            </>}
        },
        Err(err) => html! {
            { format!("{err:?}") }
        },
    }
}


// #[wasm_bindgen(js_name=initCellRender)]
// pub fn init_cell_render(mx_editor: MxEditor, mx_utils: MxUtils, div: HtmlDivElement, options: SchemaOptions) {
//     log::debug!("init cell render");

//     let props = CellComponentProps {
//             context: MxGraphContext {
//                 api_url: options.api_url.unwrap_or("undefiend".to_owned()).into(),
//                 mx_utils,
//                 mx_editor
//             }.into()
//         };


//     yew::Renderer::<CellComponent>::with_root_and_props(div.into(), props).render();
// }

// static GLOBAL_DATA: Mutex<Option<AppHandle<CellInfoComponent>>> = Mutex::new(None);
// lazy_static! {
//     static ref AAAAA: RwLock<RefCell<Option<AppHandle<CellInfoComponent>>>> = RefCell::new(None);
// }

#[wasm_bindgen(js_name=renderCellInfo)]
pub fn render_cell_info(mx_cell: MxCell, mx_editor: MxEditor, mx_utils: MxUtils, div: HtmlDivElement, options: SchemaOptions)
{
    // let meta = mx_cell.get_meta().unwrap_or_default();

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
            }.into()
        };

    let handle: AppHandle<CellInfoComponent> = yew::Renderer::<CellInfoComponent>::with_root_and_props(div.into(), props).render();

    handle.destroy();
    // Dispatch::<store::cell::State>::global().set(store::cell::State {
    //     meta,
    //     ..Default::default()
    // });

    log::info!("cell loaded");
}
