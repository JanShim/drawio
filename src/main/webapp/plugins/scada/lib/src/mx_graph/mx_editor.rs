use wasm_bindgen::prelude::*;



#[wasm_bindgen]
extern "C" {
    pub fn name() -> String;

    pub type MxEditor;
}

#[cfg(test)]
impl MxEditor {
    pub fn new() -> Self {
        Self {
            obj: JsValue::undefined()
        }
    }
}