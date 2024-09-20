use std::rc::Rc;

use implicit_clone::{unsync::IString, ImplicitClone};
use serde::{Deserialize, Serialize};
use yew::Reducible;
use yewdux::Reducer;

use crate::{errors::CellStateError, model::cell_meta::CellMeta, store::cell};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ImplicitClone)]
#[serde(rename = "ds")]
pub struct DataSourceMeta {
    #[serde(rename="@tag")]
    pub tag: IString,
    #[serde(rename="@path")]
    pub path: IString,
}

impl Default for DataSourceMeta {
    fn default() -> Self {
        Self { 
            tag: Default::default(),
            path: Default::default(),
        }
    }
}

/// reducer's Action
pub enum DataSourceAction {
    SetTag(IString),
    SetPath(IString),
    Set{tag: IString, path: IString},
}

/// Reducible
impl Reducible for DataSourceMeta {
    /// Reducer Action Type
    type Action = DataSourceAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let curr = (*self).clone();
        match action {
            DataSourceAction::SetTag(tag) => Self { tag, ..curr }.into(),
            DataSourceAction::SetPath(path) => Self { path, ..curr }.into(),
            DataSourceAction::Set{tag, path} => Self { tag, path }.into(),
        }
    }
}

pub struct WidgetApplyDsAction(pub DataSourceMeta);
impl Reducer<cell::CellState> for WidgetApplyDsAction {
    fn apply(self, state: Rc<cell::CellState>) -> Rc<cell::CellState> {
        let mut widget = state.meta.widget.clone()
            .expect(format!("{}", CellStateError::NotWidget).as_str());

        widget.data_source = self.0;

        cell::CellState {
            cell: state.cell.clone(),
            meta: CellMeta { 
                    widget: Some(widget),
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
    fn xml_state_meta_serde_works() {
        let item = DataSourceMeta {
            tag: "proba".into(),
            ..Default::default()
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<DataSourceMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    

}