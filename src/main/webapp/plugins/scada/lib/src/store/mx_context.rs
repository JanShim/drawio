use std::rc::Rc;

use wasm_bindgen::JsValue;
use web_sys::Node;
use yew::AttrValue;

use crate::{model::{mx_editor::MxEditor, mx_utils::MxUtils}, utils::get_graph_svg};

pub type TMxGraphContext = Rc<MxGraphContext>;

#[derive(Clone, PartialEq, Debug)]
pub struct MxGraphContext {
    pub api_url: AttrValue,
    pub mx_utils: MxUtils,
    pub mx_editor: MxEditor,    
}

impl MxGraphContext {

    // pub fn get_cell_style(&self) -> Result<AttrValue, JsValue> {
    //     self.cell.get_style()
    //         .map(|o| o.into())
    //         .ok_or(JsValue::from("no cell"))
    // }

    // pub fn set_cell_style(&self, style: String) {
    //     self.cell.set_style(style);
    // }        

    pub fn get_graph_xml(&self) -> Result<Node, JsValue> {
        self.mx_editor.get_graph_xml()
    }

    pub fn get_graph_svg(&self) -> String {
        get_graph_svg(&self.mx_editor).into()
    }    

    pub fn get_xml(&self, node: Node) -> Result<Option<String>, JsValue> {
        self.mx_utils.get_xml(node)
    }

    // pub fn get_diagram_bounding_box(&self) -> Result<JsValue, JsValue>
    // {
    //     self.mx_editor.get_diagram_bounding_box()
    // }    
    
}

impl Default for MxGraphContext {
    fn default() -> Self {
        Self { 
            api_url: Default::default(),
            mx_utils: Default::default(),
            mx_editor: Default::default(),
        }
    }
}