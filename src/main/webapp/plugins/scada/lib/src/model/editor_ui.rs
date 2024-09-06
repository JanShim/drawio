use wasm_bindgen::prelude::*;
// use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    pub fn name() -> String;

    pub type EditorUi;

    // /**
    //  * Returns the current page and XML for the given page.
    //  */
    // // EditorUi.prototype.getDiagramSnapshot = function()
    // // {
    // // 	return {node: this.editor.getGraphXml()};
    // // };    
    // #[wasm_bindgen(method, js_name=getDiagramSnapshot)]
    // pub fn get_diagram_snapshot(this: &EditorUi) -> JsValue;
}

impl EditorUi {


    
}

impl PartialEq for EditorUi {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}

impl std::fmt::Debug for EditorUi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EditorUi").field("obj", &self.obj).finish()
    }
}

