use std::rc::Rc;
use common_model::{multystate::{range::{RangeType, RangeValue}, state::StateXml, state_predef::StatePredefType, MultystateXml}, traits::PredefStyle};
use implicit_clone::unsync::IString;
use wasm_bindgen::JsValue;
use yew::AttrValue;
use yewdux::{store::Store, Reducer};

use crate::model::{
    cell_meta::{
        // multystate::{state::StateMeta, MultystateMeta}, 
        CellMeta, CellMetaVariant, CellType
    }, mx_cell::MxCell, mx_editor::MxEditor, mx_utils::MxUtils
};

#[derive(Clone, PartialEq, Debug)]
pub struct State {
    pub api_url: AttrValue,
    pub cell: MxCell,
    pub meta: CellMeta,
    pub mx_utils: MxUtils,
    pub mx_editor: MxEditor,    
    pub model_node: IString,
}

impl State {
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

    pub fn set_cell_style(&self, style: String) {
        self.cell.set_style(style);
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

impl Default for State {
    fn default() -> Self {
        Self { 
            api_url: Default::default(),
            cell: Default::default(),
            meta: Default::default(),
            mx_utils: Default::default(),
            mx_editor: Default::default(),
            model_node: Default::default(),
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
        log::debug!("CellState  {:?}", self);

        self != old
        || self.cell != old.cell
        || self.meta != old.meta
    }
}

// ========= reducers =================
pub struct SetCellTypeAction(pub CellType);
impl Reducer<State> for SetCellTypeAction {
    fn apply(self, state: Rc<State>) -> Rc<State> {
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
        
        State {
            meta,
            ..(*state).clone()
        }.into()
    }
}

pub struct SetCellModelAction(pub IString);
impl Reducer<State> for SetCellModelAction {
    fn apply(self, state: Rc<State>) -> Rc<State> {
        // let model = state.mx_utils.parse_xml(self.0.to_string()).unwrap();
        log::debug!("SetCellModelAction model: {:?}", self.0);

        State {
            model_node: self.0,
            ..(*state).clone()
        }.into()        
    }
}

pub struct ApplyStateAction(pub StateXml);
impl Reducer<State> for ApplyStateAction {
    fn apply(self, state: Rc<State>) -> Rc<State> {
        if let CellMetaVariant::Multystate(multystate) = &mut state.meta.data.clone()  {
            let new_state = self.0;            
            let index = new_state.get_index();
            let states = &mut multystate.states;
            states[index] = StateXml { ..new_state };

            return  State {
                    meta: CellMeta { 
                        data: CellMetaVariant::Multystate(multystate.clone()), 
                        ..state.meta.clone() 
                    },
                    ..(*state).clone()
                }
                .into();
        }
        state
    }
}

pub struct ApplyPredefStateStyleAction {
    pub r#type: StatePredefType, 
    pub style: IString,
}
impl Reducer<State> for ApplyPredefStateStyleAction {
    fn apply(self, state: Rc<State>) -> Rc<State> {
        if let CellMetaVariant::Multystate(curr) = state.meta.data.clone()  {
            let mut curr_predef_item = match self.r#type {
                    StatePredefType::Default => curr.predef[0].clone(),
                    StatePredefType::Bad => curr.predef[1].clone(),
                };  
            curr_predef_item.set_style(self.style);

            let predef = match self.r#type {
                    StatePredefType::Default => vec![curr_predef_item, curr.predef[1].clone()],
                    StatePredefType::Bad => vec![curr.predef[0].clone(), curr_predef_item],
                };

            let data = CellMetaVariant::Multystate(MultystateXml { predef, ..curr });
            return State {
                    meta: CellMeta { data, ..state.meta.clone() },
                    ..(*state).clone()
                }
                .into();
        }
        state
    }
}

pub struct MultystateAddStateAction;
impl Reducer<State> for MultystateAddStateAction {
    fn apply(self, state: Rc<State>) -> Rc<State> {
        if let CellMetaVariant::Multystate(multystate) = &mut state.meta.data.clone() {
            match multystate.range_type {
                RangeType::DISCRET => {
                    let prev = multystate.states.last()
                        .map(|o| o.value.get_value())
                        .unwrap_or(0);
    
                    multystate.states.push(StateXml { 
                        pk: multystate.states.len(), 
                        name: format!("state-{}", multystate.states.len()).into(),
                        value: RangeValue::DiscretConst { value: prev },
                        ..Default::default() 
                    })
                },
                RangeType::RANGE => {
                    let prev = multystate.states.last()
                        .map(|o| o.value.get_to())
                        .unwrap_or(0.0);
    
                    multystate.states.push(StateXml { 
                        pk: multystate.states.len(), 
                        name: format!("state-{}", multystate.states.len()).into(),
                        value: RangeValue::RangeConst { from: prev, to: prev },
                        ..Default::default() 
                    })
                },            
            };
    
            return State {
               meta: CellMeta { 
                    data: CellMetaVariant::Multystate(multystate.clone()),
                    ..state.meta.clone() 
                },
                ..(*state).clone()
            }
            .into()
        }
        log::error!("can't add state for not multystate");
        state
    }
}

