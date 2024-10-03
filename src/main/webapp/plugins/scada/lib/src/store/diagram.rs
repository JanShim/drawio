use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::Node;
use yewdux::Store;

use crate::model::{common::ModelForm, mx_editor::MxEditor, mx_utils::MxUtils};

#[derive(Store, Clone, PartialEq, Debug)]
pub struct State {
    pub api_url: String,
    pub mx_utils: Option<Rc<MxUtils>>,
    pub mx_editor: Option<Rc<MxEditor>>,
    pub model_meta: ModelForm,
}

impl State {
    pub fn get_graph_xml(&self) -> Result<Node, JsValue> {
        self.mx_editor.clone()
            .map(|editor| editor.get_graph_xml())
            .unwrap_or(Err(JsValue::from("no editor error")))
    }

    pub fn get_xml(&self, node: Node) -> Result<Option<String>, JsValue> {
        self.mx_utils.clone()
            .map(|utils | utils.get_xml(node))
            .unwrap_or(Err(JsValue::from("no utils error")))
    }
}

impl Default for State {
    fn default() -> Self {
        Self { 
            api_url: Default::default(),
            mx_utils: None,
            mx_editor: None,
            model_meta: Default::default(),
        }
    }
}