use wasm_bindgen::prelude::*;
use web_sys::Element;
use quick_xml::{
    de::from_str, 
    se::to_string,
};

use crate::{errors::CellStateError, model::scada_diagram, schema_app::js_functions::get_pretty_xml};

use super::cell_meta::CellMeta;

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

    pub fn get_diagram_meta(&self) -> Result<scada_diagram::meta::DiagramMeta, JsValue> {
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

    pub fn get_meta_xml(&self) -> Option<String>  {
        if let Ok(CellValue::Object(el)) = self.get_value() {
            return get_pretty_xml(el).as_string();
        }
        None
    }

    // pub fn append_meta_element<F>(&self, provider: F) -> Result<Element, JsValue>
    //     where F: FnOnce(&Element) -> &Element
    // {
    //     if let Ok(CellValue::Object(root)) = self.get_value() {
    //         // if let Some(doc) =  el.owner_document() {
    //         //     let doc.create_element("")
    //         // }
    //         let child = provider(&root);
    //         return root.append_child(child)
    //             .map(|node| match node.dyn_into::<Element>() {
    //                 Ok(e) => Ok(e),
    //                 Err(_) => Err(JsValue::from("can't convert to Element")),
    //             })?;
    //     }
    //     Err(JsValue::from_str("can't appent element to root"))
    // }

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
        let multystate = if let Some(multystate) = &meta.multystate {
                    to_string(multystate)
                        .map_err(|err| JsValue::from(err.to_string().as_str()))?
                } else {
                    String::default()
                };
        let widget = if let Some(widget) = &meta.widget {
                to_string(widget)
                    .map_err(|err| JsValue::from(err.to_string().as_str()))?
            } else {
                String::default()
            };

        Ok(format!("{widget}{multystate}"))
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
