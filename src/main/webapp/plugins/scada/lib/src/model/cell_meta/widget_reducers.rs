use std::rc::Rc;
use common_model::widget::WidgetXml;
use implicit_clone::unsync::IString;
use yewdux::Reducer;

use crate::store::cell;

use super::{CellMeta, CellMetaVariant};

pub struct WidgetUuidApplyAction(pub IString);
impl Reducer<cell::State> for WidgetUuidApplyAction {
    fn apply(self, state: Rc<cell::State>) -> Rc<cell::State> {
		todo!();

        // if let CellMetaVariant::WidgetContainer(meta) = &state.meta.data {
        //     return cell::State {
        //             meta: CellMeta { 
        //                 label: state.meta.label.clone(),
        //                 data: CellMetaVariant::WidgetContainer(WidgetXml { 
        //                     uuid: self.0, 
        //                    ..meta.clone() 
        //                 }),
        //             },
        //             ..(*state).clone()
        //         }
        //         .into();        
        // }
        // state
    }
}

pub struct WidgetMetaApplyAction(pub WidgetXml);
impl Reducer<cell::State> for WidgetMetaApplyAction {
    fn apply(self, state: Rc<cell::State>) -> Rc<cell::State> {
		todo!();

        // if let CellMetaVariant::WidgetContainer(_) = &state.meta.data {
        //     return cell::State {
        //             meta: CellMeta { 
        //                 label: state.meta.label.clone(),
        //                 data: CellMetaVariant::WidgetContainer(self.0),
        //             },
        //             ..(*state).clone()
        //         }
        //         .into();        
        // }
        // state
    }
}

// ==========================================================
#[cfg(test)]
mod tests {

}