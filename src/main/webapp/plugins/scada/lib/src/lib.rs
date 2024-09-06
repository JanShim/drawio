use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

mod utils;
mod errors;
mod model;
mod cell_app;
mod schema_app;
mod scada_object;
mod components;
mod store;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    log::info!("wasm loaded");
    Ok(())
}