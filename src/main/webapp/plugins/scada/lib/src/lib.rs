use js_sys::{Array, Reflect};
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use web_sys:: HtmlDivElement;
use wasm_bindgen::JsValue;

use mx_graph::{
    mx_cell_highlight::MxCellHighlight, 
    mx_editor::MxEditor, 
    mx_cell::MxCell,
};

use app::App;

mod mx_graph;
mod app;
mod utils;

#[wasm_bindgen]
pub fn greet(a: &str) ->  Result<String, JsValue> {
    Ok(format!("Hello, {}!", a))
}

// #[wasm_bindgen]
// pub fn set_cell(container: HtmlDivElement, cell: JsValue) -> Result<(), JsValue> {

//     let cell = serde_wasm_bindgen::from_value::<MxCell>(cell)?;
//     log::debug!("cell: {:?}", cell);

//     let window = web_sys::window().expect("no global `window` exists");
//     let document = window.document().expect("should have a document on window");            
//     let div = document.create_element("div").unwrap();
//     div.set_text_content(Some("AAAAAAAAAAA"));
//     container.append_child(&div).unwrap();


//     Ok(())

//     // match container.dyn_into::<HtmlDivElement>() {
//     //     Ok(root) => {
//     //         let cell = serde_wasm_bindgen::from_value::<MxCell>(cell)?;
//     //         log::debug!("cell: {:?}", cell);
//     //         Ok(())
//     //     }
//     //     Err(_) => Err(JsValue::from_str("argument not a HtmlDivElement")),
//     // }
// }


// #[wasm_bindgen]
// pub fn set_unselected(container: HtmlDivElement, highlight: MxCellHighlight, wrapper: Wrapper) -> Result<(), JsValue> {

// // // let dialog = Reflect::get(&app, &JsValue::from_str("dialog")).unwrap();
// // // let open = Reflect::get(&dialog, &JsValue::from_str("open")).unwrap();
// // // let arr = Array::new();
// // // Reflect::apply(open, &JsValue::from_str("alert"), &arr);

// //     let method_name = "highlight";

// //     let method: js_sys::Function = match Reflect::get(&highlight, &JsValue::from_str(method_name)) {
// //         Ok(value) if value.is_function() => Ok(value.into()),
// //         _ => Err(JsValue::from_str(format!("highlight object doesn't have a suitable {} method", method_name).as_str())),
// //     }?;

// //     Reflect::apply(&method, &JsValue::from_str("alert"), &arr);

//     // highlight.highlight(None);

//     // let method: js_sys::Function = match Reflect::get(&wrapper, &JsValue::from_str("test")) {
//     //     Ok(value) if value.is_function() => Ok(value.into()),
//     //     _ => Err(JsValue::from_str(format!("highlight object doesn't have a suitable {} method", "test").as_str())),
//     // }?;

//     // let arr = Array::new();
//     // Reflect::apply(&method, &wrapper, &arr)?;
    
//     wrapper.test();

//     container.set_inner_html(format!("<p><i>{}</i></p>", "ничего не выбрано").as_str());

//     Ok(())
// }


#[wasm_bindgen]
pub fn cell_click(cell: &MxCell) {

    log::info!("cell clicked {:?}", cell);
}


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    log::info!("wasm loaded");

    // let window = web_sys::window().expect("no global `window` exists");
    // let document = window.document().expect("should have a document on window");
    

    // let elem = document.get_element_by_id("container").expect("must be id=container");
    // let div = document.create_element("div").unwrap();
    // div.set_text_content(Some("AAAAAAAAAAA"));
    // elem.append_child(&div).unwrap();
    // log::info!("{}", elem.inner_html());

    Ok(())
}