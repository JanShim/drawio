use std::rc::Rc;
use implicit_clone::unsync::IString;
use wasm_bindgen::JsValue;
use yewdux::{store::{self, Store}, Reducer};

use crate::{errors::CellStateError, model::{
    cell_meta::{
        // multystate::{state::StateMeta, MultystateMeta}, 
        CellMeta, CellMetaVariant, CellType
    }, 
    mx_cell::MxCell
}};

#[derive(Clone, PartialEq, Debug)]
// #[store(storage = "local")]
pub struct CellState {
    pub cell: MxCell,
    pub meta: CellMeta,
}

impl CellState {
    // pub fn set_meta_from_self(&mut self) -> Result<(), JsValue> {
    //     if let Some(meta) = self.cell.get_meta().ok()
    //     {
    //         self.meta = meta;
    //         return Ok(());
    //     };
    //     Err(CellStateError::NoMeta.into())
    // }

    // pub fn apply_meta_to_cell(&self) {
    //     if let Some(cell) = &self.cell {
    //         let meta = &self.meta;
    //         cell.set_meta(meta).ok();
    //     }        
    // }

    // pub fn get_ref_meta(&self) -> Result<&CellMeta, JsValue> {
    //     Ok(&self.meta)
    // }   

    // pub fn get_multystate_state<'a>(&'a self, index: usize) -> Result<&'a StateMeta, JsValue> {
    //     log::debug!("get_multystate_state index = {index}");

    //    if let Some(multy) = &self.meta.multystate {
    //         let states = &multy.states;
    //         log::debug!("get_multystate_state: {:#?}", states);
    //         if index < states.len() {
    //             log::debug!("get_multystate_state[{index}] = {:#?}", states[index]);
    //             return Ok(&states[index]);
    //         }
    //         return Err(JsValue::from("index not in range"));
    //    };
    //    Err(JsValue::from_str("no multisate in cell"))
    // }

    // pub fn get_multystate(&self) -> Result<&MultystateMeta, JsValue> {
    //     self.meta.get_multystate()
    // }

    // pub fn get_mut_multystate(&mut self) -> Result<&mut MultystateMeta, JsValue> {
    //     self.meta.get_mut_multystate()
    // }

    // pub fn set_multystate(&mut self, ms: MultystateMeta)  {
    //     self.meta.multystate.replace(ms);
    // }

    // pub fn get_multystate_data_source(&self) ->  Result<&DataSource, JsValue> {
    //     self.meta.get_multystate()
    //         .map(|ms| &ms.data_source)
    // }

    // pub fn set_multystate_data_source(&mut self, ds: DataSource) ->  Result<(), JsValue> {
    //     self.meta.get_mut_multystate()
    //         .map(|ms| {
    //             ms.set_data_source(ds);
    //         })
    // } 

    pub fn get_cell_style(&self) -> Result<IString, JsValue> {
        self.cell.get_style()
            .map(|o| o.into())
            .ok_or(JsValue::from("no cell"))
    }

    // pub fn set_multystate_state_style(&self, i: usize, style: IString) -> Result<(), JsValue> {
    //     log::debug!("set_multystate_state_style: multy {:?}", self.meta.multystate);
    //     if let Some(multy) = self.meta.multystate.clone() {
    //         let mut states = multy.states;
    //         if i < states.len() {

    //             let state = &mut states[i];
    //             state.set_style(style);
    //             return Ok(());
    //         }
    //         return Err(CellStateError::MultyStateStateIndexError{index: i, len: states.len()}.into());

    //     } 
    //     Err(CellStateError::NoMeta().into())
    //     // let multy = self.meta.multystate.clone().ok_or::<JsValue>(CellStateError::NoMeta().into())?;
    // }

}

impl Default for CellState {
    fn default() -> Self {
        Self { 
            cell: Default::default(),
            meta: Default::default(),
        }
    }
}

impl Store for CellState {
    fn new(_cx: &yewdux::Context) -> Self {
        Self {
            ..Default::default()
        }
    }
    
    fn should_notify(&self, old: &Self) -> bool {
        log::debug!("check changed {} {} {}", self != old, self.cell != old.cell, self.meta != old.meta);
        log::debug!("CellState  {:?}", self);

        self != old
        || self.cell != old.cell
        || self.meta != old.meta
    }
}

// pub struct SetStyleAction(pub IString);
// impl Reducer<CellState> for SetStyleAction {
//     fn apply(self, state: Rc<CellState>) -> Rc<CellState> {
//         log::debug!("{}", self.0);
//         CellState {
//             cell: state.cell.clone(),
//             meta: state.meta.clone(),
//         }.into()
//     }
// }

pub struct SetCellTypeAction(pub CellType);
impl Reducer<CellState> for SetCellTypeAction {
    fn apply(self, state: Rc<CellState>) -> Rc<CellState> {
        let meta = match self.0 {
            CellType::MULTYSTATE => CellMeta {
                label: state.cell.get_label().into(),
                data: CellMetaVariant::Multystate(Default::default()),
            },
            CellType::VALUE => CellMeta {
                label: state.cell.get_label().into(),
                data: CellMetaVariant::Value(Default::default()),
            },
            _ => Default::default(),
        };
        
        CellState {
            cell: state.cell.clone(),
            meta,
        }.into()
    }
}


///// reducer's Action
// pub enum Action {
//     SetStyle(IString),
// }

// impl Reducible for CellState {
//     type Action = Action;
    
//     fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
//         match action {
//             Action::SetStyle(style) => Self {
//                 cell: self.cell.clone(),     
//                 meta: self.meta.clone(),           
//             }.into(),
//             _ => self
//         }
//     }

// }