use wasm_bindgen::prelude::*;
use web_sys::{Element, Node};

use crate::model::scada_diagram;

#[wasm_bindgen]
extern "C" {
    pub fn name() -> String;

    pub type MxCell;

    /**
     * Function: getValue
     *
     * Returns the user object of the cell. The user
     * object is stored in <value>.
     */
    //mxCell.prototype.getValue = function()
    #[wasm_bindgen(method, js_name=getValue)]
    fn mx_get_value(this: &MxCell) -> JsValue;    

}

impl MxCell {
    pub fn get_diagram_meta(&self) -> Result<scada_diagram::meta::Meta, JsValue> {
        match self.mx_get_value() {
            str if str.is_string() => Ok(Default::default()),
            elem if elem.is_object() => elem.dyn_into::<Element>().map(|e| e.into()),
            _ => Err("can't create diagram meta".into()),           
        }
    }
}


impl PartialEq for MxCell {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}

impl std::fmt::Debug for MxCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MxCell").field("obj", &self.obj).finish()
    }
}

