use wasm_bindgen::JsValue;
use yewdux::store::Store;

use crate::{components::multystate::state, errors::CellStateError, model::{
    cell_meta::{
        multystate::{
            data_source::DataSource, state::StateMeta, MultystateMeta 
        }, 
        CellMeta
    }, 
    mx_cell::MxCell
}};

#[derive(Clone, PartialEq, Debug)]
// #[store(storage = "local")]
pub struct State {
    pub cell: Option<MxCell>,
    pub meta: CellMeta,
}

impl State {
    pub fn set_meta_from_self(&mut self) {
        if let Some(cell) = &self.cell {
           if let Some(meta) = cell.get_meta().ok()
                // .or_else(|err| { log::error!("{err:#?}"); Err(()) }).ok()
            {
                self.meta = meta;
                return;
            };
        }

        self.meta = Default::default();
    }

    pub fn apply_meta_to_cell(&self) {
        if let Some(cell) = &self.cell {
            let meta = &self.meta;
            cell.set_meta(meta).ok();
        }        
    }


    pub fn get_ref_meta(&self) -> Result<&CellMeta, JsValue> {
        Ok(&self.meta)
    }   

    pub fn get_multystate_state<'a>(&'a self, index: usize) -> Result<&'a StateMeta, JsValue> {
        log::debug!("get_multystate_state index = {index}");

       if let Some(multy) = &self.meta.multystate {
            let states = &multy.states;
            log::debug!("get_multystate_state: {:#?}", states);
            if index < states.len() {
                log::debug!("get_multystate_state[{index}] = {:#?}", states[index]);
                return Ok(&states[index]);
            }
            return Err(JsValue::from("index not in range"));
       };
       Err(JsValue::from_str("no multisate in cell"))
    }

    pub fn get_multystate(&self) -> Result<&MultystateMeta, JsValue> {
        self.meta.get_multystate()
    }

    pub fn get_mut_multystate(&mut self) -> Result<&mut MultystateMeta, JsValue> {
        self.meta.get_mut_multystate()
    }

    pub fn get_multystate_data_source(&self) ->  Result<&DataSource, JsValue> {
        self.meta.get_multystate()
            .map(|ms| &ms.data_source)
    }

    pub fn set_multystate_data_source(&mut self, ds: DataSource) ->  Result<(), JsValue> {
        self.meta.get_mut_multystate()
            .map(|ms| {
                ms.set_data_source(ds);
            })
    }    

    pub fn get_cell_style(&self) -> Result<String, JsValue> {
        self.cell.clone()
            .map(|cell| {
                cell.get_style().unwrap()
            })
            .ok_or(JsValue::from("no cell"))
    }

    // pub fn set_multystate_state_style(&self, index: usize, style: String) -> Result<(), JsValue> {
    //     let multy = self.meta.multystate.clone().ok_or::<JsValue>(CellStateError::NoMeta().into())?;
    //     let mut states = multy.states;
    //     if index < states.len() {
    //         let state = &mut states[index];
    //         state.set_style(style);
    //         log::debug!("set_multystate_state_style:!! {:?}", state);
    //         return Ok(());
    //     }

    //     Err(CellStateError::MultyStateStateIndexError{index, len: states.len()}.into())
    // }

}

impl Default for State {
    fn default() -> Self {
        Self { 
            cell: None,
            meta: Default::default(),
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