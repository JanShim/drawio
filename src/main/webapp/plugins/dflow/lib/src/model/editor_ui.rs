use wasm_bindgen::prelude::*;
// use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    pub fn name() -> String;

    pub type EditorUi;


    /**
     * Displays a print dialog.
     */
    // EditorUi.prototype.hideDialog = function(cancel, isEsc, matchContainer)    
    #[wasm_bindgen(method, js_name=hideDialog)]
    pub fn hide_dialog(this: &EditorUi);

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

