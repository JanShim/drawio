use wasm_bindgen::prelude::*;

use crate::model::{mx_cell::MxCell,mx_editor::MxEditor};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name=loadScadaModel)]
    pub fn load_scada_model(editor: &MxEditor, xmlStr: &str);

    #[wasm_bindgen(js_name=getCell0)]
    pub fn get_cell0(editor: &MxEditor) -> MxCell;
}

