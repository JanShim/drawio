use std::rc::Rc;
use data_source::DataSourceMeta;
use implicit_clone::unsync::IString;
use serde::{Deserialize, Serialize};
use yew::Reducible;
use yewdux::Reducer;

use crate::{errors::CellStateError, store::cell};

use super::{CellMeta, CellMetaVariant};

pub mod data_source;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "widget")]
pub struct WidgetMeta {
    #[serde(rename="@uuid")]
    pub uuid: IString,
    #[serde(rename="ds", default)]
    pub data_source: DataSourceMeta,
}

impl Default for WidgetMeta {
    fn default() -> Self {
        Self { 
            uuid: Default::default(), 
            data_source: Default::default() 
        }
    }
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
        if let CellMetaVariant::Widget(_) = &state.meta.data {
            return cell::CellState {
                cell: state.cell.clone(),
                meta: CellMeta { 
                        label: state.meta.label.clone(),
                        data: CellMetaVariant::Widget(self.0),
                    },
                }
                .into();        
        }
        state
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
            data_source: Default::default(),
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<WidgetMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }
   

}