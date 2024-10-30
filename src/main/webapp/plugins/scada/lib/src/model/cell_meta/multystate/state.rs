use std::rc::Rc;
use implicit_clone::unsync::IString;
use yewdux::Reducer;
use common_model::multystate::{range::RangeValue, state::StateXml};

use crate::{
     model::cell_meta::{CellMeta, CellMetaVariant}, store::cell
};


/// reducer's Action
// pub enum StateAction {
//     Clone(StateXml),
//     SetStyle(IString),
//     SetName(IString),
//     SetLinearRange(f32),
//     SetDiscretRange(u32),
// }
// impl Reducer<StateXml> for StateAction {
//     fn apply(self, state: Rc<StateXml>) -> Rc<StateXml> {
//         let curr = (*state).clone();
//         match self {
//             StateAction::SetStyle(style) => StateXml {
//                  pk: curr.pk,
//                  name: curr.name.clone(),
//                  style, 
//                  value: curr.value.clone(),
//                  selected: curr.selected,
//              }.into(),
//              StateAction::Clone(meta) => StateXml {
//                  pk: meta.pk,
//                  name: meta.name,
//                  style: meta.style, 
//                  value: meta.value,
//                  selected: meta.selected,
//              }.into(),
//              StateAction::SetName(name) => StateXml {
//                  pk: curr.pk,
//                  name,
//                  style: curr.style.clone(), 
//                  value: curr.value.clone(),
//                  selected: curr.selected,
//              }.into(),
//              StateAction::SetDiscretRange(value) => {
//                  log::debug!("discret: {value}");
//                  StateXml {
//                     pk: curr.pk,
//                     name: curr.name.clone(),
//                     style: curr.style.clone(), 
//                     value: RangeValue::DiscretConst { value },
//                     selected: curr.selected,
//                  }.into()
//             },
//              StateAction::SetLinearRange(val) => StateXml {
//                  pk: curr.pk,
//                  name: curr.name.clone(),
//                  style: curr.style.clone(), 
//                  value: RangeValue::RangeConst { from: curr.value.get_from(), to: val },
//                  selected: curr.selected,
//              }.into(),            
//         }
//     }
// }

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

