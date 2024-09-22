use yew::prelude::*;
use yewdux::use_store;
use stylist::yew::{styled_component, Global};
use wasm_bindgen::prelude::*;
use yew_hooks::use_effect_once;
use web_sys::HtmlDivElement;

use crate::{components::cell_details::CellDetailsComponent, model::mx_cell::MxCell, store::cell::{self, CellState}};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub cell_state: CellState,
}

#[styled_component(CellComponent)]
pub fn app(Props {cell_state}: &Props) -> Html {
    let (_, dispatch) = use_store::<cell::CellState>();
    let cell_state = cell_state.clone();
    use_effect_once(move || {
        log::debug!("cell-component init: {cell_state:?}");
        dispatch.set(cell_state);

        // destructor
        move || {
            dispatch.set(cell::CellState {..Default::default()});
            log::debug!("cell-component destruct");
        }
    });


    // === view items ====
    html! {
        <>
<Global css={css!(r#"
.prop-edit-panel {
    background-color: rgb(237, 244, 255);
    min-height: 20px;
}

.flex-box {
    display:flex;
    justify-content:space-between;
}

.flex-box-2 {
    display:flex;
    justify-content: flex-end;
}

.delim-label {
    background-color: #e9e9e9;
    height: 25px;
    padding: 3px;
}    
    
table.prop-table {
    width: 100%;
}
table.prop-table td {
    padding: 0px 5px 0px 5px;
    height: 25px;
    vertical-align: middle;
}
table.prop-table td input {
    width: 100%;
    height: 16px;
}
table.prop-table td.label {
    background-color: rgb(221, 221, 221);
    text-align: right;
}
table.prop-table td.img {
    width: 16px;
    padding: 0px;
}     

.img-16 {
    width: 16px;
    height: 16px;
    padding: 0px;
}

form.input-form input {
    margin: 0px 5px 0px 5px;
}    

div.svg-view {
    text-align: center;
}

"#)} />

        <div>
            <CellDetailsComponent/>
        </div>
    </>        
    }    
}


#[wasm_bindgen(js_name=renderCell)]
pub fn render_cell(div: HtmlDivElement, cell: MxCell) {
    let mut cell_state = cell::CellState {cell: Some(cell), ..Default::default()};
    cell_state.set_meta_from_self().unwrap();

    log::debug!("render_cell : {cell_state:?}");
    yew::Renderer::<CellComponent>::with_root_and_props(div.into(), Props {cell_state}).render();
}
