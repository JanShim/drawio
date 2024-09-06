use wasm_bindgen::prelude::*;
use web_sys::{js_sys::JsString, Element};

use crate::model::scada_diagram;

pub enum CellValue {
    Object(Element),
    Label(Option<String>),
}

#[wasm_bindgen]
extern "C" {
    pub fn name() -> String;
    pub type MxCell;

    /**
     * Returns the Id of the cell as a string.
     */
    #[wasm_bindgen(method, js_name=getId)]
    fn mx_get_id(this: &MxCell) -> JsValue;

    /**
     * Function: getValue
     *
     * Returns the user object of the cell. The user
     * object is stored in <value>.
     */
    //mxCell.prototype.getValue = function()
    #[wasm_bindgen(method, js_name=getValue)]
    fn mx_get_value(this: &MxCell) -> JsValue;    

    /**
     * Sets the user object of the cell. The user object
     * is stored in <value>.
     */
    //setValue(value: any): void;
    #[wasm_bindgen(method, js_name=setValue)]
    fn mx_set_value(this: &MxCell, value: JsValue);    

    /**
     * Returns a string that describes the <style>.
     */
    //getStyle(): string;    
    #[wasm_bindgen(method, js_name=getStyle)]
    fn mx_get_style(this: &MxCell) -> JsValue;
}

impl MxCell {
    pub fn id(&self) -> Option<String> {
        self.mx_get_id().as_string()
    }

    pub fn get_value(&self) -> Result<CellValue, JsValue> {
        match self.mx_get_value() {
            str if str.is_string() => Ok(CellValue::Label(str.as_string())),
            obj if obj.is_object() => obj.dyn_into::<Element>().map(|o| CellValue::Object(o)),
            null if null.is_null() => Ok(CellValue::Label(None)),
            undefiend if undefiend.is_undefined() => Ok(CellValue::Label(None)),
            _ => Err(JsValue::from_str("get value error")),
        }
    }

    pub fn mx_style(&self) -> Option<String> {
        self.mx_get_style().as_string()
    }

    pub fn get_diagram_meta(&self) -> Result<scada_diagram::meta::Meta, JsValue> {
        match self.mx_get_value() {
            str if str.is_string() => Ok(Default::default()),
            elem if elem.is_object() => elem.dyn_into::<Element>().map(|e| e.into()),
            _ => Err("can't create diagram meta".into()),           
        }
    }

    pub fn get_label(&self) -> String {
        self.get_value()
            .map(|cell_val| {
                match cell_val {
                    CellValue::Object(elem) => elem.get_attribute("label"),
                    CellValue::Label(label) => label,
                }
            })
            .map(|s| s.unwrap_or_default())
            .unwrap()
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

impl Clone for MxCell {
    fn clone(&self) -> Self {
        Self { obj: self.obj.clone() }
    }
}
