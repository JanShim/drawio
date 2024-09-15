use std::rc::Rc;

use data_source::DataSourceMeta;
use serde::{ser::Serializer, Deserialize, Deserializer, Serialize};
use state::StateMeta;
use implicit_clone::{sync::IArray, ImplicitClone};
use yew::Reducible;
use yewdux::Reducer;

pub mod state;
pub mod data_source;
pub mod state_range;

pub fn is_none_multystate(tst: &Option<MultystateMeta>) -> bool {
    match tst {
        Some(_) => false,
        None => true,
    }
}

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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ImplicitClone)]
#[serde(rename_all="lowercase")]
pub enum RangeType {
    DISCRET,
    LINIER,
}

impl Default for RangeType {
    fn default() -> Self {
        RangeType::DISCRET
    }
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
    // ApplyStates(Vec<StateMeta>),
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


/*
struct ApplyMultystateStateMeta(StateMeta);
impl Reducer<MultystateMeta> for ApplyMultystateStateMeta {
    fn apply(self, state: Rc<MultystateMeta>) -> Rc<MultystateMeta> {
        let curr = (*state).clone();
        let new_item = self.0;
        let index = new_item.pk;
        let mut states = curr.states;
        log::debug!("states {states:?}");
        states.splice(index..index+1, vec![new_item]);
        log::debug!("states {states:?}");
        MultystateMeta {
            states,
            ..curr
        }.into()        
    }
} */

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
            range_type: RangeType::LINIER,
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


}