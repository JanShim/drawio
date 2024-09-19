use yew::prelude::*;
use yewdux::use_store;
use stylist::yew::{styled_component, Global};
use wasm_bindgen::prelude::*;
use yew_hooks::use_effect_once;
use web_sys::HtmlDivElement;

use crate::{components::cell_details::CellDetailsComponent, model::mx_cell::MxCell, store::cell};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub cell: MxCell,
}

#[styled_component(CellComponent)]
pub fn app(Props {cell: mxcell}: &Props) -> Html {
    let (_, dispatch) = use_store::<cell::CellState>();

    let cell = mxcell.clone();
    let dispatcher = dispatch.clone();
    use_effect_once(move || {
        let mut new_state = cell::CellState {cell: Some(cell), ..Default::default()};
        new_state.set_meta_from_self();
        
        dispatcher.set(new_state);
        
        move || dispatcher.set(cell::CellState {..Default::default()})
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
    padding: 0px;
}

form.input-form input {
    margin: 0px 5px 0px 5px;
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
    yew::Renderer::<CellComponent>::with_root_and_props(div.into(), Props {cell}).render();
}
