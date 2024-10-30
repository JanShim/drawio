use std::rc::Rc;
use common_model::{data_source::DataSourceXml, multystate::{range::{RangeType, RangeValue}, state::StateXml, MultystateXml}};
use yewdux::Reducer;

use crate::store::cell;

use super::{CellMeta, CellMetaVariant};

pub mod state;
pub mod state_predef;

// ---------------- reducer's Action
pub enum MultystateMetaAction {
    CreateState,
    ApplyDataSource(DataSourceXml),
    ApplyMultystateStateMeta(StateXml),
}
impl Reducer<MultystateXml> for MultystateMetaAction {
    fn apply(self, state: Rc<MultystateXml>) -> Rc<MultystateXml> 
    {
        let curr = (*state).clone();
        match self {
            MultystateMetaAction::CreateState => MultystateXml {
                states: {
                    let mut states = curr.states.clone();
                    states.push(StateXml { pk: states.len(), ..Default::default() }); 
                    states
                },
                ..curr
            }.into(),
            MultystateMetaAction::ApplyDataSource(data_source) => MultystateXml { data_source, ..curr }.into(),
            MultystateMetaAction::ApplyMultystateStateMeta(state) => { 
                let index = state.pk;
                let mut states = curr.states;
                log::debug!("states before {states:?}");
                states.splice(index..index+1, vec![state]);
                log::debug!("states after {states:?}");
                MultystateXml {
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

