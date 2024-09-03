use wasm_bindgen::prelude::*;
use web_sys::Node;
// use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    pub fn name() -> String;

    pub type MxEditor;

    /**
     * Returns the current page and XML for the given page.
     */
    #[wasm_bindgen(method, js_name=getGraphXml)]
    pub fn mx_get_graph_xml(this: &MxEditor) -> JsValue;

    /**
     * Sets the XML node for the current diagram.
     */
    #[wasm_bindgen(method, js_name=setGraphXml)]
    pub fn set_graph_xml(this: &MxEditor, node: Node);

}

impl MxEditor {
    pub fn get_graph_xml(&self) -> Result<Node, JsValue> {
        let node = self.mx_get_graph_xml();
        match node {
            node if node.is_object() => node.dyn_into::<Node>(),
            err => Err(err),
        }
    }
}

impl PartialEq for MxEditor {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}

impl std::fmt::Debug for MxEditor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MxEditor").field("obj", &self.obj).finish()
    }
}

