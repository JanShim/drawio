use std::rc::Rc;

use data_source::DataSourceMeta;
use implicit_clone::unsync::IString;
use serde::{Deserialize, Serialize};
use yew::Reducible;
use yewdux::Reducer;

use crate::{errors::CellStateError, store::cell};

use super::CellMeta;

pub mod data_source;

pub fn is_none_widget(tst: &Option<WidgetMeta>) -> bool {
    match tst {
        Some(_) => false,
        None => true,
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "widget")]
pub struct WidgetMeta {
    #[serde(rename="@uuid")]
    pub uuid: IString,
    #[serde(rename="ds", default)]
    pub data_source: DataSourceMeta,
}

/// reducer's Action
pub enum Action {
    SetUuid(IString),
}

/// Reducible
impl Reducible for WidgetMeta {
    /// Reducer Action Type
    type Action = Action;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let curr = (*self).clone();
        match action {
            Action::SetUuid(uuid) => Self { uuid, ..curr }.into(),
        }
    }
}


pub struct WidgetApplyAction(pub WidgetMeta);
impl Reducer<cell::CellState> for WidgetApplyAction {
    fn apply(self, state: Rc<cell::CellState>) -> Rc<cell::CellState> {
        // let mut widget = state.meta.widget.clone()
        //     .expect(format!("{}", CellStateError::NotWidget).as_str());

        // widget.data_source = self.0;

        cell::CellState {
            cell: state.cell.clone(),
            meta: CellMeta { 
                    widget: Some(self.0),
                    ..state.meta.clone() 
                },
            }
            .into()            
    }
}

// ==========================================================
#[cfg(test)]
mod tests {
    use quick_xml::{
        de::from_str,
        se::to_string,
    };

    use super::*;

    #[test]
    fn xml_widget_meta_serde_works() {
        let item = WidgetMeta {
            uuid: "some".into(),
            data_source: todo!(),
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<WidgetMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }
   

}