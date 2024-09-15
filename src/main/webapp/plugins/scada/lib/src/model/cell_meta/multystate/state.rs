use std::rc::Rc;

use serde::{Deserialize, Serialize};
use yew::Reducible;
use implicit_clone::{unsync::IString, ImplicitClone};



#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ImplicitClone)]
#[serde(rename = "state")]
pub struct StateMeta {
    #[serde(rename = "@pk")]
    pub pk: usize,
    #[serde(rename = "@name")]
    pub name: IString,
    #[serde(rename = "@style")]
    pub style: IString,
    #[serde(skip)]
    pub selected: bool,
}

impl StateMeta {
    pub fn set_style(&mut self, style: IString) {
        self.style = style;
        log::debug!("set_style:!! {:?}", self.style);
    }

    pub fn get_index(&self) -> usize {
        self.pk
    }
}

impl Default for StateMeta {
    fn default() -> Self {
        Self { 
            pk: Default::default(),
            name: "наименование".into(),
            style: "".into(),
            selected: false,
        }
    }
}

/// reducer's Action
pub enum StateAction {
    Clone(StateMeta),
    SetStyle(IString),
}

impl Reducible for StateMeta {
    type Action = StateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
           StateAction::SetStyle(style) => Self {
                pk: self.pk,
                name: self.name.clone(),
                style, 
                selected: self.selected,
            }.into(),
            StateAction::Clone(meta) => Self {
                pk: meta.pk,
                name: meta.name.clone(),
                style: meta.style.clone(), 
                selected: meta.selected,
            }.into(),
           _ => self
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
        let item = StateMeta {
            pk: 0,
            ..Default::default()
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<StateMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    

}