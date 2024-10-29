use std::rc::Rc;
use yew::Reducible;
use implicit_clone::unsync::IString;
use serde::{Deserialize, Serialize};
use yewdux::Reducer;

use crate::store::cell;

use super::{data_source::DataSourceMeta, CellMeta, CellMetaVariant};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(rename = "value")]
pub struct ValueMeta {
    pub ds: DataSourceMeta,

    // #[serde(rename="@tag")]
    // pub tag: IString,
    // #[serde(rename="@path")]
    // pub path: IString,
}

// impl Default for ValueMeta {
//     fn default() -> Self {
//         Self { 
//             tag: Default::default(), 
//             path: Default::default() 
//         }
//     }
// }


/// reducer's Action
pub enum ValueAction {
    SetTag(IString),
    SetPath(IString),
    Set{tag: IString, path: IString},
}

/// Reducible
impl Reducible for ValueMeta {
    /// Reducer Action Type
    type Action = ValueAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let curr = (*self).clone();
        match action {
            ValueAction::SetTag(tag) => Self { ds: DataSourceMeta { tag, ..self.ds.clone() } }.into(),
            ValueAction::SetPath(path) => Self { ds: DataSourceMeta { path, ..self.ds.clone() } }.into(),
            ValueAction::Set{tag, path} => Self { ds: DataSourceMeta { tag, path } }.into(),
        }
    }
}

pub struct ApplyValueMetaAction(pub ValueMeta);
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

// ==========================================================
#[cfg(test)]
mod tests {
    use quick_xml::{
        de::from_str,
        se::to_string,
    };

    use super::*;

    #[test]
    fn xml_value_meta_serde_works() {
        let item = ValueMeta {
            ..Default::default()
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<ValueMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }
   

}