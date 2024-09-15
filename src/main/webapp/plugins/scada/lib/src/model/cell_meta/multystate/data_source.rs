use std::rc::Rc;

use implicit_clone::{unsync::IString, ImplicitClone};
use serde::{Deserialize, Serialize};
use yew::Reducible;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ImplicitClone)]
#[serde(rename = "ds")]
pub struct DataSource {
    #[serde(rename="@tag")]
    pub tag: IString,
    #[serde(rename="@path")]
    pub path: IString,
}

impl Default for DataSource {
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
impl Reducible for DataSource {
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
        let item = DataSource {
            tag: "proba".into(),
            ..Default::default()
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<DataSource>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    

}