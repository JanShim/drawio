use yewdux::store::Store;

use crate::model::{cell_meta::CellMeta, mx_cell::MxCell};

#[derive(Clone, PartialEq)]
// #[store(storage = "local")]
pub struct State {
    pub cell: Option<MxCell>,
    pub meta: Option<Box<CellMeta>>,
    pub entries: Vec<i32>,
}

impl State {
    pub fn set_meta_from_self(&mut self) {
        if let Some(cell) = &self.cell {
            let meta = cell.get_meta()
                .or_else(|err| { log::error!("{err:#?}"); Err(()) }).ok()
                .map(|o| Box::new(o));

            self.meta = meta;
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self { 
            cell: None,
            meta: None,
            entries: vec![1,2,3],
        }
    }
}

impl Store for State {
    fn new(_cx: &yewdux::Context) -> Self {
        Self {
            ..Default::default()
        }
    }
    
    fn should_notify(&self, old: &Self) -> bool {
        log::debug!("check changed {} {} {}", self != old, self.cell != old.cell, self.meta != old.meta);

        self != old
        || self.cell != old.cell
        || self.meta != old.meta
    }
}