use std::rc::Rc;
use serde::{ Deserialize, Serialize};
use state::{StateJson, StateMeta};
use implicit_clone::ImplicitClone;
use state_range::{RangeValue, RangeType};
use yew::Reducible;
use yewdux::Reducer;

use crate::store::cell;

use super::{data_source::{DataSourceJson, DataSourceMeta}, CellMeta, CellMetaVariant};

pub mod state;
// pub mod data_source;
pub mod state_range;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(rename = "multystate")]
pub struct MultystateJson {
    #[serde(rename="range-type")]
    pub range_type: RangeType,
    pub ds: DataSourceJson,
    pub states: Vec<StateJson>,
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ImplicitClone)]
#[serde(rename = "multystate")]
pub struct MultystateMeta {
    #[serde(rename="@range-type", default)]
    pub range_type: RangeType,
    #[serde(rename="ds", default)]
    pub data_source: DataSourceMeta,
    #[serde(rename="state", default)]
    pub states: Vec<StateMeta>,
}

impl MultystateMeta {
    pub fn create_state(&mut self) {
        self.states.push(StateMeta {
            pk: self.states.len(), 
            ..Default::default()
        });
    }

    pub fn set_data_source(&mut self, ds: DataSourceMeta) {
        self.data_source = ds;
    }

    // pub fn set_state<'a>(&'a self, meta: &'a StateMeta) {
    //     if let Some(index) = meta.pk.parse::<usize>().ok() {
    //        let mut ietm =  self.states[index];
            
    //     }
    // }
}

impl Default for MultystateMeta {
    fn default() -> Self {
        Self { 
            range_type: Default::default(), 
            data_source: Default::default(),
            states: vec![],
        }
    }
}

impl From<MultystateJson> for MultystateMeta {
    fn from(MultystateJson { range_type, ds, states }: MultystateJson) -> Self 
    {
        Self { 
            range_type, 
            data_source: ds.into(), 
            states: states.iter().map(|o| o.clone().into()).collect(), 
        }
    }
}
// ------------ SetDataSource ------------------
pub struct SetDataSource(pub DataSourceMeta);
impl Reducer<MultystateMeta> for SetDataSource {
    fn apply(self, state: Rc<MultystateMeta>) -> Rc<MultystateMeta> {
        MultystateMeta {
            data_source: self.0,
            range_type: state.range_type.clone(),
            states: state.states.clone(),
        }.into()
    }
}

// ---------------- reducer's Action
pub enum MultystateMetaAction {
    CreateState,
    ApplyDataSource(DataSourceMeta),
    ApplyMultystateStateMeta(StateMeta),
}

impl Reducible for MultystateMeta {
    /// Reducer Action Type
    type Action = MultystateMetaAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let curr = (*self).clone();
        match action {
            MultystateMetaAction::CreateState => Self {
                states: {
                    let mut states = curr.states.clone();
                    states.push(StateMeta { pk: states.len(), ..Default::default() }); 
                    states
                },
                ..curr
            }.into(),
            MultystateMetaAction::ApplyDataSource(data_source) => Self { data_source, ..curr }.into(),
            // MultystateMetaAction::ApplyStates(states) => Self { states, ..curr }.into(),
            MultystateMetaAction::ApplyMultystateStateMeta(state) => { 
                let index = state.pk;
                let mut states = curr.states;
                log::debug!("states before {states:?}");
                states.splice(index..index+1, vec![state]);
                log::debug!("states after {states:?}");
                Self {
                    states,
                    ..curr
                }.into()
            },
        }
    }
}

pub struct MultystateAddStateAction;
impl Reducer<cell::State> for MultystateAddStateAction {
    fn apply(self, state: Rc<cell::State>) -> Rc<cell::State> {
        if let CellMetaVariant::Multystate(multystate) = &mut state.meta.data.clone() {
            match multystate.range_type {
                RangeType::DISCRET => {
                    let prev = multystate.states.last()
                        .map(|o| o.value.get_value())
                        .unwrap_or(0);
    
                    multystate.states.push(StateMeta { 
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
    
                    multystate.states.push(StateMeta { 
                        pk: multystate.states.len(), 
                        name: format!("state-{}", multystate.states.len()).into(),
                        value: RangeValue::RangeConst { from: prev, to: prev },
                        ..Default::default() 
                    })
                },            
            };
    
            return cell::State {
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

// ==========================================================
#[cfg(test)]
mod tests {
    use quick_xml::{
        de::from_str,
        se::to_string,
    };

    use super::*;

    #[test]
    fn xml_multystate_meta_nostates_serde_works() {
        let item = MultystateMeta::default();

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<MultystateMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }

    #[test]
    fn xml_multystate_meta_states_serde_works() {
        let item = MultystateMeta {
            range_type: RangeType::RANGE,
            data_source: DataSourceMeta { 
                tag: "tag".into(), 
                path: "path".into(),
            },
            states: vec![
                StateMeta {
                    pk: 1,
                    name: "name-1".into(),
                    ..Default::default()
                },
                StateMeta {
                    pk: 2,
                    name: "name-1".into(),
                    ..Default::default()
                },
            ],
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<MultystateMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }

    #[test]
    fn from_xml_works() {
        let xml = r#"      <multystate range-type="discret">
        <ds tag="" path="" />
        <state pk="0" name="state-0" style="triangle;flipH=0;strokeWidth=5;rounded=0;perimeterSpacing=0;aspect=fixed;points=[[0,0.5,0,0,0]];fillColor=#666666;">
          <discret-const value="0" />
        </state>
        <state pk="1" name="state-1" style="triangle;flipH=0;strokeWidth=5;rounded=0;perimeterSpacing=0;aspect=fixed;points=[[0,0.5,0,0,0]];fillColor=#CCCCCC;strokeColor=#FF0000;">
          <discret-const value="1" />
        </state>
        <state pk="2" name="state-2" style="triangle;flipH=0;strokeWidth=5;rounded=0;perimeterSpacing=0;aspect=fixed;points=[[0,0.5,0,0,0]];fillColor=#00FF00;strokeColor=#000000;">
          <discret-const value="2" />
        </state>
        <state pk="3" name="state-3" style="triangle;flipH=0;strokeWidth=5;rounded=0;perimeterSpacing=0;aspect=fixed;points=[[0,0.5,0,0,0]];fillColor=#FFFF00;strokeColor=#000000;">
          <discret-const value="3" />
        </state>
      </multystate>"#;


      let item = from_str::<MultystateMeta>(xml).unwrap();
      println!("{item:?}");


    }

    #[test]
    fn json_deser_works() {
        let json = r#"{"range-type":"discret","ds":{"tag":"","path":""},"states":[{"pk":0,"name":"","style":"","value":{"type":"discret-const","value":0}},{"pk":0,"name":"","style":"","value":{"type":"discret-const","value":0}}]}"#;

        let multy = serde_json::from_str::<MultystateJson>(json).unwrap();
        let multy: MultystateMeta = multy.into();
        println!("{multy:?}");
    }


}