use std::{cmp::Ordering, collections::HashSet, rc::Rc};
use common_model::multystate::range::RangeType;
use implicit_clone::unsync::IString;
use wasm_bindgen::JsValue;
use yewdux::{store::Store, Reducer};

use crate::model::{
    cell_meta::{ CellMeta, CellMetaVariant, CellType, }, 
    mx_cell::MxCell,
};

pub const NOT_CELL: &str = "not cell";
pub const NOT_CELL_META: &str = "not cell meta";
pub const NO_CONTEXT_FOUND: &str = "no ctx found";

#[derive(Clone, PartialEq, Debug, Store)]
pub struct State {
    pub cell: Option<Rc<MxCell>>,
    pub meta: CellMeta,
    pub model_node: IString,
    pub start_apply: bool,
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

    // pub fn get_cell_label(&self) -> Result<AttrValue, JsValue> {
    //     let cell = self.cell.clone().ok_or(JsValue::from(NOTCELL))?;
    //     Ok(cell.get_label().into())
    // }

    pub fn get_cell_style(&self) -> Result<IString, JsValue> {
        let cell = self.cell.clone().ok_or(JsValue::from(NOT_CELL))?;
        cell.get_style()
            .map(|o| o.into())
            .ok_or(JsValue::from(NOT_CELL))
    }

    pub fn set_cell_style(&self, style: String) {
        if let Some(mut cell) = self.cell.clone() {
            Rc::make_mut(&mut cell).set_style(style);
        }
    }    

    pub fn get_state_meta(&self) -> CellMeta {
        self.meta.clone()
    }

//     pub fn set_cell_meta(&self, meta: &CellMeta) -> Result<CellMeta, JsValue> {
//         let cell = self.cell.clone().ok_or(JsValue::from(NOT_CELL))?;
//         if let Ok(CellValue::Object(el)) = cell.get_value() {
//            el.set_attribute("label", meta.label.as_str()).ok();

//            let inner_html = cell.get_meta_inner_html(&meta)?;
//            log::debug!("set_inner_html {:#?}", inner_html);
//            el.set_inner_html(inner_html.as_str());

//            return cell.get_meta();
//         }
//         Err(JsValue::from_str("can't set cell meta data"))        
//    }

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
            cell: None,
            meta: CellMeta::default(),
            model_node: Default::default(),
            start_apply: false,
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
//         // log::debug!("check changed {} {} {}", self != old, self.cell != old.cell, self.meta != old.meta);
//         // log::debug!("CellState  {:?}", self);

//         let tst = self != old
//             || self.cell != old.cell
//             || self.meta != old.meta;

//         log::debug!("should_notify? {tst}");

//         self != old
//         || self.cell != old.cell
//         || self.meta != old.meta
        
//     }
// }

pub fn cell_type_compare(a: &CellType, b: &CellType) -> Ordering {
    let a = (*a).clone() as u8;
    let b = (*b).clone() as u8;
    a.cmp(&b)
}
// ========= reducers =================
pub struct SetCellTypeAction(pub HashSet<CellType>);
impl Reducer<State> for SetCellTypeAction {
    fn apply(self, state: Rc<State>) -> Rc<State> {
        let mut cell_types = self.0.into_iter().collect::<Vec<_>>();
        cell_types.sort_by(cell_type_compare);

        let data = cell_types.into_iter()
            .map(|o| match o {
                CellType::LABEL =>  CellMetaVariant::Label(Default::default()),
                CellType::MULTYSTATE => CellMetaVariant::Multystate(Default::default()),
                CellType::WIDGETCONTAINER =>  CellMetaVariant::WidgetContainer(Default::default()),
            })
            .collect::<Vec<_>>();

        let cell = state.cell.clone().ok_or(JsValue::from(NOT_CELL)).unwrap();
        let meta = CellMeta{ 
                label: cell.get_label().into(), 
                types: data 
            };

        // assigne meta to editor cell
        let res = cell.set_meta(&meta);
        if res.is_err() {
            log::error!("{:?}", res.err().unwrap().as_string())
        }
        
        // return
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
        // log::debug!("SetCellModelAction model: {:?}", self.0);

        State {
            model_node: self.0,
            ..(*state).clone()
        }.into()        
    }
}

pub struct StartApplyAction(pub bool);
impl Reducer<State> for StartApplyAction {
    fn apply(self, state: Rc<State>) -> Rc<State> {
        State {
            start_apply: self.0,
            ..(*state).clone()
        }.into()
    }
}

// pub struct SetLabelAction(pub LabelValueXml);
// impl Reducer<State> for SetLabelAction {
//     fn apply(self, state: Rc<State>) -> Rc<State> {
//         let mut new_meta = state.meta.clone();
//         new_meta.set_label_meta(self.0);

//         log::debug!("NEW LABEL {:?}", new_meta);            

//         State {
//             // meta: CellMeta { 
//             //     label: new_meta.label.clone(), 
//             //     types: new_meta.types.clone(),
//             // },
//             meta: new_meta,
//             ..(*state).clone()
//         }.into()
//     }
// }

// pub struct SetMultystateAction(pub MultystateXml);
// impl Reducer<State> for SetMultystateAction {
//     fn apply(self, state: Rc<State>) -> Rc<State> {
//         let mut new_meta = state.meta.clone();
//         new_meta.set_multystate_meta(self.0);

//         log::debug!("NEW MULTY {:?}", new_meta);

//         State {
//             meta: new_meta,
//             ..(*state).clone()
//         }.into()
//     }
// }

pub struct SetRangeTypeAction(pub RangeType);
impl Reducer<State> for SetRangeTypeAction {
    fn apply(self, state: Rc<State>) -> Rc<State> {
        let mut meta =  state.meta.clone();
        if let Ok(mut multystate) = meta.get_multystate_meta() {
            multystate.states = vec![];
            multystate.range_type = self.0;
            meta.set_multystate_meta(multystate);
            
            log::debug!("new range: {:?}", meta);

            return State { 
                meta, 
                ..(*state).clone()
            }.into();
        };

        // else return
        state
    }
}

// pub struct ApplyStateAction(pub StateXml);
// impl Reducer<State> for ApplyStateAction {
//     fn apply(self, state: Rc<State>) -> Rc<State> {
//         todo!();

//         // if let CellMetaVariant::Multystate(multystate) = &mut state.meta.data.clone()  {
//         //     let new_state = self.0;            
//         //     let index = new_state.get_index();
//         //     let states = &mut multystate.states;
//         //     states[index] = StateXml { ..new_state };

//         //     return  State {
//         //             meta: CellMeta { 
//         //                 data: CellMetaVariant::Multystate(multystate.clone()), 
//         //                 ..state.meta.clone() 
//         //             },
//         //             ..(*state).clone()
//         //         }
//         //         .into();
//         // }
//         // state
//     }
// }

// pub struct ApplyPredefStateStyleAction {
//     pub r#type: StatePredefType, 
//     pub style: IString,
// }
// impl Reducer<State> for ApplyPredefStateStyleAction {
//     fn apply(self, state: Rc<State>) -> Rc<State> {
// 		todo!();

//         // if let CellMetaVariant::Multystate(curr) = state.meta.data.clone()  {
//         //     let mut curr_predef_item = match self.r#type {
//         //             StatePredefType::Default => curr.predef[0].clone(),
//         //             StatePredefType::Bad => curr.predef[1].clone(),
//         //         };  
//         //     curr_predef_item.set_style(self.style);

//         //     let predef = match self.r#type {
//         //             StatePredefType::Default => vec![curr_predef_item, curr.predef[1].clone()],
//         //             StatePredefType::Bad => vec![curr.predef[0].clone(), curr_predef_item],
//         //         };

//         //     let data = CellMetaVariant::Multystate(MultystateXml { predef, ..curr });
//         //     return State {
//         //             meta: CellMeta { data, ..state.meta.clone() },
//         //             ..(*state).clone()
//         //         }
//         //         .into();
//         // }
//         // state
//     }
// }

// pub struct MultystateAddStateAction;
// impl Reducer<State> for MultystateAddStateAction {
//     fn apply(self, state: Rc<State>) -> Rc<State> {
//         if let Ok(multystate) =  state.meta.get_multystate() {
//             let mut multystate = (*multystate).borrow_mut();
//             let pk = multystate.states.len();
//             let name: IString = format!("state-{}", multystate.states.len()).into();
//             match multystate.range_type {
//                 RangeType::DISCRET => {
//                     let prev = multystate.states.last()
//                         .map(|o| o.value.get_value())
//                         .unwrap_or(0);

//                     let state = StateXml { 
//                         pk, name,
//                         value: RangeValue::DiscretConst { value: prev },
//                         ..Default::default() 
//                     };
//                     multystate.push_state(state);
//                 },
//                 RangeType::RANGE => {
//                     let prev = multystate.states.last()
//                         .map(|o| o.value.get_to())
//                         .unwrap_or(0.0);
    
//                     let state = StateXml { 
//                         pk, name,
//                         value: RangeValue::RangeConst { from: prev, to: prev },
//                         ..Default::default() 
//                     };
//                     multystate.push_state(state);
//                 },            
//             };
    
//             // state.meta.borrow_mut().set_multystate_meta(multystate);

//             return State {
//                meta: CellMeta { 
//                     // types: CellMetaVariant::Multystate(multystate.clone()),
//                     ..state.meta.clone() 
//                 },
//                 ..(*state).clone()
//             }
//             .into()

//         }
//         log::error!("can't add state for not multystate");
//         state
//     }
// }

// #[derive(Clone, PartialEq, Debug)]
// pub struct App {
//     pub curr: AppHandle<CellComponent>,
// }