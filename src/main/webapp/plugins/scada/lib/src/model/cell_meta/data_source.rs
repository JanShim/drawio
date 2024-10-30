use std::rc::Rc;
use common_model::{data_source::DataSourceXml, multystate::MultystateXml, widget::WidgetXml};
use implicit_clone::unsync::IString;
use yewdux::Reducer;

use crate::store::cell;

use super::{CellMeta, CellMetaVariant};

// ------------------- reducer's Action
// pub enum DataSourceAction {
//     SetTag(IString),
//     SetPath(IString),
//     Set{tag: IString, path: IString},
// }
// impl Reducer<DataSourceXml> for DataSourceAction {
//     fn apply(self, state: Rc<DataSourceXml>) -> Rc<DataSourceXml> {
//         let curr = (*state).clone();
//         match self {
//             DataSourceAction::SetTag(tag) => DataSourceXml { tag, ..curr }.into(),
//             DataSourceAction::SetPath(path) => DataSourceXml { path, ..curr }.into(),
//             DataSourceAction::Set{tag, path} => DataSourceXml { tag, path }.into(),
//         }
//     }
// }

pub struct WidgetApplyDsAction(pub DataSourceXml);
impl Reducer<cell::State> for WidgetApplyDsAction {
    fn apply(self, state: Rc<cell::State>) -> Rc<cell::State> {
        if let CellMetaVariant::Widget(widget) = &state.meta.data {
            return cell::State {
                    meta: CellMeta { 
                        label: state.meta.label.clone(), 
                        data: CellMetaVariant::Widget(WidgetXml {
                            data_source: self.0,
                            ..widget.clone()
                        }),
                    },
                    ..(*state).clone()
                }.into();
        };

        state
    }
}

// ------------ reducer SetDataSource ------------------
pub struct SetDataSource(pub DataSourceXml);
impl Reducer<MultystateXml> for SetDataSource {
    fn apply(self, state: Rc<MultystateXml>) -> Rc<MultystateXml> {
        MultystateXml {
            data_source: self.0,
            range_type: state.range_type.clone(),
            states: state.states.clone(),
            predef: state.predef.clone(),
            // bad: state.bad.clone(),
        }.into()
    }
}


// ==========================================================
#[cfg(test)]
mod tests {

}