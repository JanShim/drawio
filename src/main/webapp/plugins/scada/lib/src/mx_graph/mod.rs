use wasm_bindgen::prelude::*;

pub mod mx_editor;
pub mod mx_cell;
pub mod mx_cell_highlight;
pub mod mx_cell_state;


#[wasm_bindgen]
extern "C" {
    pub fn name() -> String;

    pub type MxGraph;
}