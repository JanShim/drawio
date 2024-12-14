use yewdux::Store;

use crate::model::common::ModelForm;

#[derive(Store, Clone, PartialEq, Debug)]
pub struct State {
    // pub redraw: bool,
    pub model_meta: ModelForm,
}

impl State {
    // pub fn get_graph_xml(&self) -> Result<Node, JsValue> {
    //     self.mx_editor.get_graph_xml()
    // }

    // pub fn get_graph_svg(&self) -> String {
    //     get_graph_svg(&self.mx_editor).into()
    // }

    // pub fn get_xml(&self, node: Node) -> Result<Option<String>, JsValue> {
    //     self.mx_utils.get_xml(node)
    // }

    // pub fn get_diagram_bounding_box(&self) -> Result<JsValue, JsValue>
    // {
    //     self.mx_editor.get_diagram_bounding_box()
    // }
}

impl Default for State {
    fn default() -> Self {
        Self {
            // redraw: Default::default(),
            model_meta: Default::default(),
        }
    }
}