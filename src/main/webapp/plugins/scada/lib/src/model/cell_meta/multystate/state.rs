use std::rc::Rc;

use serde::{Deserialize, Serialize};
use yew::Reducible;
use implicit_clone::unsync::IString;



#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
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
    pub fn set_style(&mut self, style: String) {
        self.style = style;
    }

    pub fn get_index(&self) -> Option<usize> {
        self.pk.parse::<usize>().ok()
    }
}

impl Default for StateMeta {
    fn default() -> Self {
        Self { 
            pk: Default::default(),
            name: "наименование".to_owned(),
            style: "".to_owned(),
            selected: false,
        }
    }
}

/// reducer's Action
pub enum StateAction {
    Clone(Rc<StateMeta>),
    SetStyle(IString),
}

impl Reducible for StateMeta {
    type Action = StateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
           StateAction::SetStyle(style) => {
            let aaa = Self {
                pk: self.pk.clone(),
                name: self.name.clone(),
                style: style.into(), 
                selected: self.selected,
            };
        
                log::debug!("StateAction::SetStyle {style}, {aaa:?}");
                aaa.into()
            },
            StateAction::Clone(meta) => Self {
                pk: meta.pk.clone(),
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