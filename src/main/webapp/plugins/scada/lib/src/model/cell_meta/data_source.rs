use std::rc::Rc;

use implicit_clone::{unsync::IString, ImplicitClone};
use serde::{Deserialize, Serialize};
use yew::Reducible;
use yewdux::Reducer;

use crate::store::cell;

use super::{widget::WidgetMeta, CellMeta, CellMetaVariant};


// ------- DataSourceMeta ------------------
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(rename = "ds")]
pub struct DataSourceJson {
    pub tag: String,
    pub path: String,
}

// ------- DataSourceMeta ------------------
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

impl From<DataSourceJson> for DataSourceMeta {
    fn from( DataSourceJson { tag, path }: DataSourceJson) -> Self {
        Self { 
            tag: tag.into(),
            path: path.into(),
        }
    }
}

// ------------------- reducer's Action
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
impl Reducer<cell::State> for WidgetApplyDsAction {
    fn apply(self, state: Rc<cell::State>) -> Rc<cell::State> {
        if let CellMetaVariant::Widget(widget) = &state.meta.data {
            return cell::State {
                    meta: CellMeta { 
                        label: state.meta.label.clone(), 
                        data: CellMetaVariant::Widget(WidgetMeta {
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

    #[test]
    fn from_json_works() {
        let item = DataSourceJson {
            tag: "tag".into(),
           ..Default::default()
        };

        let item: DataSourceMeta = item.into();

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<DataSourceMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }   

}