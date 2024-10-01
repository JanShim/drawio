use std::rc::Rc;
use yewdux::Store;

use crate::model::{mx_editor::MxEditor, mx_utils::MxUtils};

#[derive(Store, Clone, PartialEq, Debug)]
pub struct State {
    pub api_url: String,
    pub mx_utils: Option<Rc<MxUtils>>,
    pub mx_editor: Option<Rc<MxEditor>>,
}

impl Default for State {
    fn default() -> Self {
        Self { 
            api_url: Default::default(),
            mx_utils: None,
            mx_editor: None,
        }
    }
}