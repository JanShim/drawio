use yewdux::Store;

use crate::model::mx_cell::MxCell;

#[derive(Clone, PartialEq, Store)]
pub struct State {
    pub count: i32,
    pub cell: Option<MxCell>,
}

impl State {
    pub fn inc(&mut self)  {
        self.count += 1;
    }

    pub fn dec(&mut self)  {
        self.count -= 1;
    }
}

impl Default for State {
    fn default() -> Self {
        Self { 
            count: Default::default(), 
            cell: None,
        }
    }
}

// impl Store for State {
//     fn new(_cx: &yewdux::Context) -> Self {
//         Self {
//             ..Default::default()
//         }
//     }
    
//     fn should_notify(&self, old: &Self) -> bool {
//         self.cell != old.cell
//     }
// }