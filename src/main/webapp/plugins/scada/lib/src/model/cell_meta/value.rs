use std::rc::Rc;
use implicit_clone::unsync::IString;
use yewdux::Reducer;

use common_model::{data_source::DataSourceXml, value::ValueXml};

use crate::store::cell;

use super::{CellMeta, CellMetaVariant};

/// reducer's Action
pub enum ValueAction {
    SetTag(IString),
    SetPath(IString),
    Set{tag: IString, path: IString},
}
impl Reducer<ValueXml> for ValueAction {
    fn apply(self, state: Rc<ValueXml>) -> Rc<ValueXml> {
        let curr = (*state).clone();
        match self {
            ValueAction::SetTag(tag) => ValueXml { ds: DataSourceXml { tag, ..curr.ds.clone() } }.into(),
            ValueAction::SetPath(path) => ValueXml { ds: DataSourceXml { path, ..curr.ds.clone() } }.into(),
            ValueAction::Set{tag, path} => ValueXml { ds: DataSourceXml { tag, path } }.into(),
        }
    }
}


pub struct ApplyValueMetaAction(pub ValueXml);
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

