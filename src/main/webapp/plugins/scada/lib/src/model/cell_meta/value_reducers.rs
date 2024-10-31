use std::rc::Rc;
use implicit_clone::unsync::IString;
use yewdux::Reducer;

use common_model::{data_source::DataSourceXml, free_value::FreeValueXml};

use crate::store::cell;

use super::{CellMeta, CellMetaVariant};

/// reducer's Action
pub enum ValueAction {
    SetTag(IString),
    SetPath(IString),
    Set{tag: IString, path: IString},
}
impl Reducer<FreeValueXml> for ValueAction {
    fn apply(self, state: Rc<FreeValueXml>) -> Rc<FreeValueXml> {
        let curr = (*state).clone();
        match self {
            ValueAction::SetTag(tag) => FreeValueXml { ds: DataSourceXml { tag, ..curr.ds.clone() } }.into(),
            ValueAction::SetPath(path) => FreeValueXml { ds: DataSourceXml { path, ..curr.ds.clone() } }.into(),
            ValueAction::Set{tag, path} => FreeValueXml { ds: DataSourceXml { tag, path } }.into(),
        }
    }
}


pub struct ApplyValueMetaAction(pub FreeValueXml);
impl Reducer<cell::State> for ApplyValueMetaAction {
    fn apply(self, state: Rc<cell::State>) -> Rc<cell::State> {
        if let CellMetaVariant::Value(_) = state.meta.data {
            return cell::State {
                meta: CellMeta {
                    data: CellMetaVariant::Value(self.0),
                    ..state.meta.clone()
                }, 
                ..(*state).clone()
            }.into();
                
        }
        state
    }
}

