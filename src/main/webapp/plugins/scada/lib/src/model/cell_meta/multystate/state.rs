use std::rc::Rc;

use serde::{Deserialize, Serialize};
use web_sys::FormData;
use yew::Reducible;
use implicit_clone::{unsync::IString, ImplicitClone};
use yewdux::Reducer;

use crate::{
     model::cell_meta::{CellMeta, CellMetaVariant}, store::cell
};

use super::state_range::{RangeType, RangeValue, RangeValueJson};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default, ImplicitClone)]
#[serde(rename = "state")]
pub struct StateJson {
    #[serde(rename = "pk")]
    pub pk: usize,
    #[serde(rename = "name")]
    pub name: IString,
    #[serde(rename = "style")]
    pub style: IString,
    #[serde(rename = "value")]
    pub value: RangeValueJson,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ImplicitClone)]
#[serde(rename = "state")]
pub struct StateXml {
    #[serde(rename = "@pk")]
    pub pk: usize,
    #[serde(rename = "@name")]
    pub name: IString,
    #[serde(rename = "@style")]
    pub style: IString,
    #[serde(rename = "$value")]
    pub value: RangeValue,
    #[serde(skip)]
    pub selected: bool,
}

impl StateXml {
    pub fn set_style(&mut self, style: IString) {
        self.style = style;
        log::debug!("set_style:!! {:?}", self.style);
    }

    pub fn get_index(&self) -> usize {
        self.pk
    }


}

impl Default for StateXml {
    fn default() -> Self {
        Self { 
            pk: Default::default(),
            name: "state".into(),
            style: "".into(),
            selected: false,
            value: Default::default(),
        }
    }
}

impl From<FormData> for StateXml {
    fn from(data: FormData) -> Self {
        let range_type = match data.get("range-type").as_string() {
            Some(value) => match value {
                _ if value=="discret" => RangeType::DISCRET,
                _ => RangeType::RANGE,
            },
            None => RangeType::DISCRET,
        };
        
        let range = match range_type {
            RangeType::DISCRET => {
               let value = data.get("value").as_string().map(|s| s.parse::<u32>().unwrap()).unwrap();
               RangeValue::DiscretConst { value }
            },
            RangeType::RANGE => {
               let from =  data.get("from").as_string().map(|s| s.parse::<f32>().unwrap()).unwrap();
               let to =  data.get("value").as_string().map(|s| s.parse::<f32>().unwrap()).unwrap();
               RangeValue::RangeConst {from, to}
            },
        };

        Self {
            pk: data.get("pk").as_string().map(|s| s.parse::<usize>().unwrap()).unwrap(),
            name: data.get("name").as_string().unwrap().into(),
            style: Default::default(),      // will by replased later
            selected: false,
            value: range,
        }
    }
}

impl From<StateJson> for StateXml {
    fn from(StateJson { pk, name, style, value }: StateJson) -> Self {
        Self { 
            pk, 
            name, 
            style, 
            value: value.into(), 
            selected: false, 
        }
    }
}

/// reducer's Action
pub enum StateAction {
    Clone(StateXml),
    SetStyle(IString),
    SetName(IString),
    SetLinearRange(f32),
    SetDiscretRange(u32),
}

impl Reducible for StateXml {
    type Action = StateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
           StateAction::SetStyle(style) => Self {
                pk: self.pk,
                name: self.name.clone(),
                style, 
                value: self.value.clone(),
                selected: self.selected,
            }.into(),
            StateAction::Clone(meta) => Self {
                pk: meta.pk,
                name: meta.name,
                style: meta.style, 
                value: meta.value,
                selected: meta.selected,
            }.into(),
            StateAction::SetName(name) => Self {
                pk: self.pk,
                name,
                style: self.style.clone(), 
                value: self.value.clone(),
                selected: self.selected,
            }.into(),
            StateAction::SetDiscretRange(value) => {
                log::debug!("discret: {value}");
                Self {
                pk: self.pk,
                name: self.name.clone(),
                style: self.style.clone(), 
                value: RangeValue::DiscretConst { value },
                selected: self.selected,
            }.into()},
            StateAction::SetLinearRange(val) => Self {
                pk: self.pk,
                name: self.name.clone(),
                style: self.style.clone(), 
                value: RangeValue::RangeConst { from: self.value.get_from(), to: val },
                selected: self.selected,
            }.into(),            
       }
    }
}

pub struct MultystateApplyStateAction(pub StateXml);
impl Reducer<cell::State> for MultystateApplyStateAction {
    fn apply(self, state: Rc<cell::State>) -> Rc<cell::State> {
        if let CellMetaVariant::Multystate(multystate) = &mut state.meta.data.clone()  {
            let new_state = self.0;            
            let index = new_state.get_index();
            let states = &mut multystate.states;
            states[index] = StateXml { ..new_state };

            return  cell::State {
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
        let item = StateXml {
            pk: 0,
            ..Default::default()
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<StateXml>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    

    #[test]
    fn json_state_meta_serde_works() {
        let item = StateJson {
            pk: 0,
            ..Default::default()
        };

        let str = serde_json::to_string(&item).unwrap();
        println!("{str}");

        let meta = serde_json::from_str::<StateJson>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }      

    #[test]
    fn range_linear_serde_works() {
        let item = StateXml {
            pk: 0,
            value: RangeValue::RangeConst { from: 1.0, to: 2.0, },
            ..Default::default()
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<StateXml>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    

    #[test]
    fn json_range_linear_serde_works() {
        let item = StateJson {
            pk: 0,
            value: RangeValueJson::RangeConst { from: 1.0, to: 2.0, },
            ..Default::default()
        };

        let str = serde_json::to_string(&item).unwrap();
        println!("{str}");

        let meta = serde_json::from_str::<StateJson>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    

    #[test]
    fn from_works() {
        let item = StateJson {
            pk: 0,
            value: RangeValueJson::RangeConst { from: 1.0, to: 2.0, },
            ..Default::default()
        };

        let item: StateXml = item.into();

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<StateXml>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }           

}