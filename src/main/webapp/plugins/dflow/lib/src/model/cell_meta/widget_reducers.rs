use std::rc::Rc;
use common_model::widget::WidgetXml;
use implicit_clone::unsync::IString;
use yewdux::Reducer;

use crate::store::cell;

use super::{CellMeta, DFlowVariant};

// pub struct WidgetUuidApplyAction(pub IString);
// impl Reducer<cell::State> for WidgetUuidApplyAction {
//     fn apply(self, state: Rc<cell::State>) -> Rc<cell::State> {
//         if let Ok(meta) = &state.meta.get_widget_container_meta() {
//             return cell::State {
//                     meta: CellMeta { 
//                         types: vec![CellMetaVariant::WidgetContainer(meta.clone())],
//                         ..state.meta.clone()
//                     },
//                     ..(*state).clone()
//                 }
//                 .into();        
//         }
//         state
//     }
// }

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