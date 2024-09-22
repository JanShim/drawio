use std::rc::Rc;

use serde::{Deserialize, Serialize};
use web_sys::FormData;
use yew::Reducible;
use implicit_clone::{unsync::IString, ImplicitClone};
use yewdux::Reducer;

use crate::{
     errors::CellStateError, model::cell_meta::{CellMeta, CellMetaVariant}, store::cell
};

use super::state_range::{Range, RangeType};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ImplicitClone)]
#[serde(rename = "state")]
pub struct StateMeta {
    #[serde(rename = "@pk")]
    pub pk: usize,
    #[serde(rename = "@name")]
    pub name: IString,
    #[serde(rename = "@style")]
    pub style: IString,
    #[serde(rename = "$value")]
    pub range: Range,
    #[serde(skip)]
    pub selected: bool,
}

impl StateMeta {
    pub fn set_style(&mut self, style: IString) {
        self.style = style;
        log::debug!("set_style:!! {:?}", self.style);
    }

    pub fn get_index(&self) -> usize {
        self.pk
    }


}

impl Default for StateMeta {
    fn default() -> Self {
        Self { 
            pk: Default::default(),
            name: "state".into(),
            style: "".into(),
            selected: false,
            range: Default::default(),
        }
    }
}

impl From<FormData> for StateMeta {
    fn from(data: FormData) -> Self {
        let range_type = match data.get("range-type").as_string() {
            Some(value) => match value {
                _ if value=="discret" => RangeType::DISCRET,
                _ => RangeType::LINEAR,
            },
            None => RangeType::DISCRET,
        };
        
        let range = match range_type {
            RangeType::DISCRET => {
               let value = data.get("value").as_string().map(|s| s.parse::<u32>().unwrap()).unwrap();
               Range::Discret { value }
            },
            RangeType::LINEAR => {
               let from =  data.get("from").as_string().map(|s| s.parse::<f32>().unwrap()).unwrap();
               let to =  data.get("value").as_string().map(|s| s.parse::<f32>().unwrap()).unwrap();
               Range::Linear {from, to}
            },
        };

        Self {
            pk: data.get("pk").as_string().map(|s| s.parse::<usize>().unwrap()).unwrap(),
            name: data.get("name").as_string().unwrap().into(),
            style: Default::default(),      // will by replased later
            selected: false,
            range,
        }
    }
}

/// reducer's Action
pub enum StateAction {
    Clone(StateMeta),
    SetStyle(IString),
    SetName(IString),
    SetLinearRange(f32),
    SetDiscretRange(u32),
}

impl Reducible for StateMeta {
    type Action = StateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
           StateAction::SetStyle(style) => Self {
                pk: self.pk,
                name: self.name.clone(),
                style, 
                range: self.range.clone(),
                selected: self.selected,
            }.into(),
            StateAction::Clone(meta) => Self {
                pk: meta.pk,
                name: meta.name,
                style: meta.style, 
                range: meta.range,
                selected: meta.selected,
            }.into(),
            StateAction::SetName(name) => Self {
                pk: self.pk,
                name,
                style: self.style.clone(), 
                range: self.range.clone(),
                selected: self.selected,
            }.into(),
            StateAction::SetDiscretRange(value) => {
                log::debug!("discret: {value}");
                Self {
                pk: self.pk,
                name: self.name.clone(),
                style: self.style.clone(), 
                range: Range::Discret { value },
                selected: self.selected,
            }.into()},
            StateAction::SetLinearRange(val) => Self {
                pk: self.pk,
                name: self.name.clone(),
                style: self.style.clone(), 
                range: Range::Linear { from: self.range.get_from(), to: val },
                selected: self.selected,
            }.into(),            
       }
    }
}

pub struct MultystateApplyStateAction(pub StateMeta);
impl Reducer<cell::CellState> for MultystateApplyStateAction {
    fn apply(self, state: Rc<cell::CellState>) -> Rc<cell::CellState> {
        if let CellMetaVariant::Multystate(multystate) = &mut state.meta.data.clone()  {
            let new_state = self.0;            
            let index = new_state.get_index();
            let states = &mut multystate.states;
            states[index] = StateMeta { ..new_state };

            return  cell::CellState {
                cell: state.cell.clone(),
                meta: CellMeta { 
                        data: CellMetaVariant::Multystate(multystate.clone()), 
                        ..state.meta.clone() 
                    },
                }
                .into();
        }
        state
    }
}


// pub struct MultystateApplyStateAction(StateMeta);
// impl Reducer<cell::CellState> for MultystateApplyStateAction {
//     fn apply(self, state: Rc<cell::CellState>) -> Rc<cell::CellState> {
//         let mut multystate = state.meta.multystate.clone()
//             .expect(format!("{}", CellStateError::NotMultystate).as_str());

//         let new_state = self.0;            
//         let index = new_state.get_index();
//         let states = &mut multystate.states;
//         states[index] = StateMeta { ..new_state };

//         log::debug!("states: {states:?}");

//         cell::CellState {
//             cell: state.cell.clone(),
//             meta: CellMeta { 
//                     multystate: Some(multystate), 
//                     ..state.meta.clone() 
//                 },
//             }
//             .into()            
//     }
// }


// ==========================================================
#[cfg(test)]
mod tests {
    use quick_xml::{
        de::from_str,
        se::to_string,
    };

    use super::*;

    #[test]
    fn xml_state_meta_serde_works() {
        let item = StateMeta {
            pk: 0,
            ..Default::default()
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<StateMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    

    #[test]
    fn xml_state_meta_linear_range_serde_works() {
        let item = StateMeta {
            pk: 0,
            range: Range::Linear { from: 1.0, to: 2.0, },
            ..Default::default()
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<StateMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    

}