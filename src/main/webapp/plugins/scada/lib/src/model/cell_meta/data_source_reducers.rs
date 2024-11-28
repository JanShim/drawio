use std::rc::Rc;
use common_model::{data_source::DataSourceXml, multystate::MultystateXml, widget::WidgetXml};
use yewdux::Reducer;

use crate::store::cell;

use super::{CellMeta, CellMetaVariant};

pub struct WidgetApplyDsAction(pub DataSourceXml);
impl Reducer<cell::State> for WidgetApplyDsAction {
    fn apply(self, state: Rc<cell::State>) -> Rc<cell::State> {
        todo!();

        // if let CellMetaVariant::WidgetContainer(widget) = &state.meta.data {
        //     return cell::State {
        //             meta: CellMeta { 
        //                 label: state.meta.label.clone(), 
        //                 data: CellMetaVariant::WidgetContainer(WidgetXml {
        //                     ds: self.0,
        //                     ..widget.clone()
        //                 }),
        //             },
        //             ..(*state).clone()
        //         }.into();
        // };

        // state
    }
}

// ------------ reducer SetDataSource ------------------
pub struct SetDataSource(pub DataSourceXml);
impl Reducer<MultystateXml> for SetDataSource {
    fn apply(self, state: Rc<MultystateXml>) -> Rc<MultystateXml> {
        MultystateXml {
            ds: self.0,
            range_type: state.range_type.clone(),
            predef: state.predef.clone(),
            states: state.states.clone(),
        }.into()
    }
}


// ==========================================================
#[cfg(test)]
mod tests {

}