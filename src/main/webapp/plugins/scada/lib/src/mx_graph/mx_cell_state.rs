use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn name() -> String;

    pub type MxCellState;

    //pub fn null() -> JsValue;

}