use std::rc::Rc;
use implicit_clone::unsync::IString;
use yewdux::Reducer;

use common_model::{data_source::DataSourceXml, free_value::LabelValueXml};

use crate::store::cell;

use super::{CellMeta, CellMetaVariant};

/// reducer's Action
pub enum ValueAction {
    SetTag(IString),
    SetPath(IString),
    Set{tag: IString, path: IString},
}
impl Reducer<LabelValueXml> for ValueAction {
    fn apply(self, state: Rc<LabelValueXml>) -> Rc<LabelValueXml> {
        let curr = (*state).clone();
        match self {
            ValueAction::SetTag(tag) => LabelValueXml { ds: DataSourceXml { tag, ..curr.ds.clone() } }.into(),
            ValueAction::SetPath(path) => LabelValueXml { ds: DataSourceXml { path, ..curr.ds.clone() } }.into(),
            ValueAction::Set{tag, path} => LabelValueXml { ds: DataSourceXml { tag, path } }.into(),
        }
    }
}


pub struct ApplyLabelValueMetaAction(pub LabelValueXml);
impl Reducer<cell::State> for ApplyLabelValueMetaAction {
    fn apply(self, state: Rc<cell::State>) -> Rc<cell::State> {
        let meta = state.meta.as_ref().unwrap();
        let position = meta.get_meta_position(super::CellType::LABEL);
        if position.is_some() {
            let mut new_data= meta.types.clone();
            let _ = std::mem::replace(&mut new_data[position.unwrap()], CellMetaVariant::Label(self.0).into());
            return cell::State {
                meta: Some( CellMeta {
                    types: new_data,
                    ..meta.clone()
                }), 
                ..(*state).clone()
            }.into();
        }
        // do nothing
        state
    }
}

