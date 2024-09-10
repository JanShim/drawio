use wasm_bindgen::JsValue;
use yewdux::store::Store;

use crate::model::{cell_meta::{multystate::MultystateMeta, multystate_state::StateMeta, CellMeta}, mx_cell::MxCell};

#[derive(Clone, PartialEq)]
// #[store(storage = "local")]
pub struct State {
    pub cell: Option<MxCell>,
    pub meta: CellMeta,
    pub entries: Vec<i32>,
    // pub states: Vec<StateMeta>,
}

impl State {
    pub fn set_meta_from_self(&mut self) {
        if let Some(cell) = &self.cell {
           if let Some(meta) = cell.get_meta()
                .or_else(|err| { log::error!("{err:#?}"); Err(()) }).ok()
                .map(|o| o) 
            {
                self.meta = meta;
                return;
            };
        }

        self.meta = Default::default();
    }

    pub fn get_mut_meta(&mut self) -> Result<&mut CellMeta, JsValue> {
        Ok(&mut self.meta)
    }

    pub fn get_ref_meta(&self) -> Result<&CellMeta, JsValue> {
        Ok(&self.meta)
    }   

    pub fn get_multystate_state<'a>(&'a self, id: usize) -> Result<&'a StateMeta, JsValue> {
       if let Some(multy) = &self.meta.multystate {
            let states = &multy.states;
            if id < states.len() {
                return Ok(&states[id]);
            }
            return Err(JsValue::from("index not in range"));
       };
       Err(JsValue::from_str("no multisate in cell"))
    }

    pub fn get_mut_multystate(&mut self) -> Result<&mut MultystateMeta, JsValue> {
        self.meta.get_mut_multystate()
    }

}

impl Default for State {
    fn default() -> Self {
        Self { 
            cell: None,
            meta: Default::default(),
            entries: vec![1,2,3],
            // states: Default::default(),
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