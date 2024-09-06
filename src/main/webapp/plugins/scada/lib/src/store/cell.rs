use std::rc::Rc;
use yewdux::Store;

use crate::model::mx_cell::MxCell;

#[derive(Clone, PartialEq, Store)]
struct State {
    count: i32,
    cell: Option<Rc<MxCell>>,
}

impl Default for State {
    fn default() -> Self {
        Self { 
            count: Default::default(), 
            cell: None
        }
    }
}