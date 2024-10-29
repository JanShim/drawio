use std::rc::Rc;
use implicit_clone::unsync::IString;
use serde::{Deserialize, Serialize};
use yewdux::Reducer;

use crate::store::cell;

use super::{data_source::DataSourceMeta, CellMeta, CellMetaVariant};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "widget")]
pub struct WidgetMeta {
    #[serde(rename="@uuid")]
    pub uuid: IString,
    #[serde(rename="@group", default)]
    pub group: IString,
    #[serde(rename="ds", default)]
    pub data_source: DataSourceMeta,
}

impl Default for WidgetMeta {
    fn default() -> Self {
        Self { 
            uuid: Default::default(), 
            data_source: Default::default(),
            group: Default::default(),
        }
    }
}

// /// reducer's Action
// pub enum Action {
//     SetUuid(IString),
// }

/// Reducible
// impl Reducible for WidgetMeta {
//     /// Reducer Action Type
//     type Action = Action;

//     /// Reducer Function
//     fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
//         let curr = (*self).clone();
//         match action {
//             Action::SetUuid(uuid) => {
//                 let tst = Self { uuid, ..curr };
//                 log::debug!("Action::SetUuid {tst:?}");
//                 tst.into()
//             },
//         }
//     }
// }

pub struct WidgetUuidApplyAction(pub IString);
impl Reducer<cell::State> for WidgetUuidApplyAction {
    fn apply(self, state: Rc<cell::State>) -> Rc<cell::State> {
        if let CellMetaVariant::Widget(meta) = &state.meta.data {
            return cell::State {
                    meta: CellMeta { 
                        label: state.meta.label.clone(),
                        data: CellMetaVariant::Widget(WidgetMeta { 
                            uuid: self.0, 
                           ..meta.clone() 
                        }),
                    },
                    ..(*state).clone()
                }
                .into();        
        }
        state
    }
}

pub struct WidgetMetaApplyAction(pub WidgetMeta);
impl Reducer<cell::State> for WidgetMetaApplyAction {
    fn apply(self, state: Rc<cell::State>) -> Rc<cell::State> {
        if let CellMetaVariant::Widget(_) = &state.meta.data {
            return cell::State {
                    meta: CellMeta { 
                        label: state.meta.label.clone(),
                        data: CellMetaVariant::Widget(self.0),
                    },
                    ..(*state).clone()
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
            ..Default::default()
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<WidgetMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }
   
    #[test]
    fn json_deser_works() {
        let json = r#"{"@uuid":"07c41b9b-75f9-460f-97f0-f0f0e7e93f9a","@group":"valves","ds":{"@tag":"some-tag","@path":""}}"#;

        let widget = serde_json::from_str::<WidgetMeta>(json).unwrap();
        println!("{widget:?}");

        let tst = WidgetMeta {
            uuid: "07c41b9b-75f9-460f-97f0-f0f0e7e93f9a".into(),
            group: "valves".into(),
            data_source: DataSourceMeta {
                tag: "some-tag".into(),
                path: String::new().into(),
            },
        };

        assert_eq!(widget, tst);
    }


}