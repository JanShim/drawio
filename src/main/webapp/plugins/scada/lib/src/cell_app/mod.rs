use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yewdux::use_dispatch;
use stylist::yew::{styled_component, Global};
use web_sys::HtmlDivElement;

use crate::{
    components::cell_details::CellDetailsComponent, 
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

        <CellDetailsComponent/>
    </>        
    }    
}


#[wasm_bindgen(js_name=renderCell)]
pub fn render_cell(div: HtmlDivElement, cell: MxCell) {
    yew::Renderer::<CellComponent>::with_root_and_props(div.into(), Props {cell}).render();
}
