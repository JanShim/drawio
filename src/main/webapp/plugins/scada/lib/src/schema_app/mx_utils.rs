use wasm_bindgen::prelude::*;
use web_sys::{js_sys::JsString, Node};
// use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    pub type MxUtils;
    pub fn name() -> String;

    #[wasm_bindgen(method, js_name=getPrettyXml)]
    pub fn mx_get_pretty_xml(this: &MxUtils, node: Node) -> JsValue;

//     * 
//     * Parameters:
//     * 
//     * node - DOM node to return the XML for.
//     * linefeed - Optional string that linefeeds are converted into. Default is
//     * &#xa;
//     */
//    getXml: function(node, linefeed)
    #[wasm_bindgen(method, js_name=getXml)]
    pub fn mx_get_xml(this: &MxUtils, node: Node, linefeed: JsString) -> JsValue;

    /**
     *   parseXml
     */ 
    #[wasm_bindgen(method, js_name=parseXml)]
    pub fn mx_parse_xml(this: &MxUtils, text: JsString) -> JsValue;

}

impl MxUtils {
    pub fn get_pretty_xml(&self, node: Node) -> Result<Option<String>, JsValue> {
        match self.mx_get_pretty_xml(node) {
            value if value.is_string() => Ok(value.as_string()),
            _ => Err(JsValue::from_str("can't get xml as string"))            
        }
    }

    pub fn get_xml(&self, node: Node) -> Result<Option<String>, JsValue> {
        match self.mx_get_xml(node, JsString::from("")) {
            value if value.is_string() => Ok(value.as_string()),
            _ => Err(JsValue::from_str("can't get xml as string"))            
        }
    }    

    pub fn parse_xml(&self, text: String) -> Result<Node, JsValue> {
        match self.mx_parse_xml(JsString::from(text)) {
            node if node.is_object() => node.dyn_into::<Node>(),
            _ => Err(JsValue::from_str("xml parse problems"))
        }
    }
}

impl PartialEq for MxUtils {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}

impl std::fmt::Debug for MxUtils {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MxUtils").field("obj", &self.obj).finish()
    }
}
