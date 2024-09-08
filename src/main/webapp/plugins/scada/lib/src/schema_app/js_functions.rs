use wasm_bindgen::prelude::*;
use web_sys::{js_sys::JsString, Element};

use crate::model::{mx_cell::MxCell,mx_editor::MxEditor};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name=loadScadaModel)]
    pub fn load_scada_model(editor: &MxEditor, xmlStr: &str);

    #[wasm_bindgen(js_name=getCell0)]
    pub fn get_cell0(editor: &MxEditor) -> MxCell;

    #[wasm_bindgen(js_name=getPrettyXml)]
    pub fn get_pretty_xml(el: Element) -> JsString;

}

