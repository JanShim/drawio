use std::rc::Rc;
use data_source::DataSourceMeta;
use serde::{ser::Serializer, Deserialize, Deserializer, Serialize};
use state::StateMeta;
use implicit_clone::ImplicitClone;
use state_range::{Range, RangeType};
use yew::Reducible;
use yewdux::Reducer;

use crate::{errors::CellStateError, store::cell};

use super::{CellMeta, CellMetaVariant};

pub mod state;
pub mod data_source;
pub mod state_range;

fn unwrap_states<'de, D>(deserializer: D) -> Result<Vec<StateMeta>, D::Error>
where
    D: Deserializer<'de>,
{
    /// Represents <states>...</states>
    #[derive(Deserialize, Debug, PartialEq)]
    struct List {
        #[serde(default)]
        state: Vec<StateMeta>,
    }

    Ok(List::deserialize(deserializer)?.state)
}


#[derive(Deserialize, PartialEq, Debug, Clone, ImplicitClone)]
#[serde(rename = "multystate")]
pub struct MultystateMeta {
    #[serde(rename="@range-type", default)]
    pub range_type: RangeType,
    #[serde(rename="ds", default)]
    pub data_source: DataSourceMeta,
    #[serde(deserialize_with = "unwrap_states", default)]
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

impl Serialize for MultystateMeta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        #[serde(rename = "multystate")]
        struct Root<'a> {
            #[serde(rename="@range-type", default)]
            range_type: &'a RangeType,
            #[serde(rename="ds", default)]
            pub data_source: &'a DataSourceMeta,
            states: List<'a>,
        }

        #[derive(Serialize)]
        struct List<'a> {
            state: &'a Vec<StateMeta>,
        }

        let helper = Root {
            range_type: &self.range_type,
            data_source: &self.data_source,
            states: List {
                state: &self.states,
            },
        };
        helper.serialize(serializer)
    }
}

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

/// reducer's Action
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
                        .map(|o| o.range.get_value())
                        .unwrap_or(0);
    
                    multystate.states.push(StateMeta { 
                        pk: multystate.states.len(), 
                        name: format!("state-{}", multystate.states.len()).into(),
                        range: Range::Discret { value: prev },
                        ..Default::default() 
                    })
                },
                RangeType::LINEAR => {
                    let prev = multystate.states.last()
                        .map(|o| o.range.get_to())
                        .unwrap_or(0.0);
    
                    multystate.states.push(StateMeta { 
                        pk: multystate.states.len(), 
                        name: format!("state-{}", multystate.states.len()).into(),
                        range: Range::Linear { from: prev, to: prev },
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
            range_type: RangeType::LINEAR,
            data_source: DataSourceMeta { 
                tag: "tag".into(), 
                path: "path".into(),
            },
            states: vec![
                StateMeta {
                    pk: 1,
                    name: "name-1".into(),
                    // range: Range::Linear { from: 1.0, to: 2.0 },
                    ..Default::default()
                },
                StateMeta {
                    pk: 2,
                    name: "name-1".into(),
                    // range: Range::Discret { value: 123 },
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


}