use wasm_bindgen::prelude::*;
use web_sys::Element;
use quick_xml::{
    de::from_str, 
    se::to_string,
};

use crate::{errors::CellStateError, utils::get_pretty_xml,};

use super::{cell_meta::{CellMeta, CellMetaVariant}, common,};

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

    /**
     * Function: setStyle
     *
     * Sets the string to be used as the <style>.
     */
    // mxCell.prototype.setStyle = function(style)
    #[wasm_bindgen(method, js_name=setStyle)]
    fn mx_set_style(this: &MxCell, value: String);

}

impl MxCell {
    pub fn get_id(&self) -> Option<String> {
        self.mx_get_id().as_string()
    }

    pub fn get_value(&self) -> Result<CellValue, JsValue> {
        match self.mx_get_value() {
            str if str.is_string() => Ok(CellValue::Label(str.as_string())),
            obj if obj.is_object() => obj.dyn_into::<Element>()
                .map(|o| CellValue::Object(o)),
            null if null.is_null() => Ok(CellValue::Label(None)),
            undefiend if undefiend.is_undefined() => Ok(CellValue::Label(None)),
            _ => Err(JsValue::from_str("get value error")),
        }
    }

    pub fn get_style(&self) -> Option<String> {
        self.mx_get_style().as_string()
    }

    pub fn set_style(&self, style: String) {
        self.mx_set_style(style);
    }    

    pub fn get_diagram_meta(&self) -> Result<common::DiagramMeta, JsValue> {
        match self.mx_get_value() {
            str if str.is_string() => Ok(Default::default()),
            elem if elem.is_object() => elem.dyn_into::<Element>().map(|e| e.into()),
            _ => Err("can't create diagram meta".into()),           
        }
    }

    pub fn get_widget_meta(&self) -> Result<common::DiagramMeta, JsValue> {
        match self.mx_get_value() {
            str if str.is_string() => Ok(Default::default()),
            elem if elem.is_object() => elem.dyn_into::<Element>().map(|e| e.into()),
            _ => Err("can't create widget meta".into()),           
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

    pub fn get_meta_xml(&self) -> Option<String>  {
        if let Ok(CellValue::Object(el)) = self.get_value() {
            return get_pretty_xml(el).as_string();
        }
        None
    }

    pub fn get_meta(&self) -> Result<CellMeta, JsValue> {
        match self.get_value() {
            Ok(CellValue::Object(el)) => {
                log::debug!("ELEMENT:  {:#?}", el.outer_html());
                from_str(el.outer_html().as_str())
                    .map_err(|err| JsValue::from(err.to_string().as_str()))
            },
            _ => Err(CellStateError::NoMeta.into())
        }
    }

    pub fn set_meta(&self, meta: &CellMeta) -> Result<CellMeta, JsValue> {
         if let Ok(CellValue::Object(el)) = self.get_value() {
            el.set_attribute("label", meta.label.as_str()).ok();

            let inner_html = self.get_meta_inner_html(&meta)?;
            log::debug!("set_inner_html {:#?}", inner_html);
            el.set_inner_html(inner_html.as_str());

            return self.get_meta();
         }
         Err(JsValue::from_str("can't set cell meta data"))        
    }

    pub fn get_meta_inner_html(&self, meta: &CellMeta) -> Result<String, JsValue> {
        let inner_html = meta.types.iter()
            .map(|data| {
                let outer_html = match data {
                        CellMetaVariant::Label(value) => to_string(value),
                        CellMetaVariant::Multystate(multy) => to_string(multy),
                        CellMetaVariant::WidgetContainer(widget) => to_string(widget),
                    }
                    .map_err(|err| JsValue::from(err.to_string().as_str()));
                outer_html
            })
            .collect::<Vec<_>>();

        // what if has errors
        let error = inner_html.iter().find(|o| o.is_err());
        if error.is_some() {
            let res = error.unwrap();
            let err = (*res).clone().err().unwrap();
            return Err(err);
        }

        // join results to string
        let inner_html = inner_html.into_iter()
            .map(|o| o.unwrap())
            .collect::<Vec<_>>()
            .join("");

        Ok(inner_html)
    }

}

impl Default for MxCell {
    fn default() -> Self {
        Self { obj: JsValue::null() }
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


// ==========================================================
#[cfg(test)]
mod tests {

    #[test]
    fn get_meta_inner_html_works() {

        todo!()
    }
}