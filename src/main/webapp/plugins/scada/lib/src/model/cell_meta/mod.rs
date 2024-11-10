use std::{cell::{self, RefCell}, collections::HashSet, rc::Rc};

use common_model::{free_value::LabelValueXml, multystate::MultystateXml, traits::ReplaceWith, widget::{WidgetContainerXml, WidgetXml}};
use implicit_clone::unsync::IString;
use wasm_bindgen::JsValue;
use web_sys::{FormData, Position};
use yew::Reducible;
use serde::{Deserialize, Serialize};

use crate::{errors::CellStateError, rrefcell};

pub mod data_source_reducers;
pub mod widget_reducers;
pub mod value_reducers;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CellType {
    LABEL,
    MULTYSTATE,
    WIDGETCONTAINER,
}

impl From<FormData> for CellType {
    fn from(data: FormData) -> Self {
        match data.get("cell-type").as_string() {
            Some(value) => match value {
                _ if value=="value" => CellType::LABEL,
                _ => CellType::MULTYSTATE,
            },
            None => CellType::MULTYSTATE,
        }
    }
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum CellMetaVariant {
    // #[serde(rename = "undefiend")]
    // Undefiend(UndefiendXml),
    #[serde(rename = "label")]
    Label(LabelValueXml),
    #[serde(rename = "multystate")]
    Multystate(MultystateXml),
    #[serde(rename = "widget-container")]
    WidgetContainer(WidgetContainerXml),
}

impl CellMetaVariant {

    pub fn create_state(&mut self) {
        if let Self::Multystate(multy) = self {
            multy.create_state();
        }
    }

    pub fn get_label(&self) -> Option<LabelValueXml> {
        match self {
            CellMetaVariant::Label(label) => Some(label.clone()),
            _ => None
        }
    }

    pub fn get_multystate(&self) -> Option<MultystateXml> {
        match self {
            CellMetaVariant::Multystate(multystate) => Some(multystate.clone()),
            _ => None
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "dflow")]
pub struct CellMeta {
    #[serde(rename = "@label")]
    pub label: IString,

    #[serde(rename = "$value", default)]
    pub types: Vec<CellMetaVariant>,
}

impl CellMeta {
    // pub fn find_widget_container(&self) -> Option<WidgetContainerXml>  {
    //     let items = self.data.iter()
    //         .filter(|o| {
    //             if let CellMetaVariant::WidgetContainer(_) = o {
    //                 return true;
    //             }
    //             false
    //         })
    //         .collect::<Vec<_>>();
    //     // found widget contaier
    //     if items.len() > 0  {
    //         if let CellMetaVariant::WidgetContainer(item) = items[0] {
    //             return Some(item.clone());
    //         }
    //         return None;
    //     }
    //     // result
    //     None
    // }

    // pub fn find_multystate(&self) -> Option<MultystateXml>  {
    //     let items = self.data.iter()
    //         .filter(|o| {
    //             if let CellMetaVariant::Multystate(_) = o {
    //                 return true;
    //             }
    //             false
    //         })
    //         .collect::<Vec<_>>();
    //     // found Multystate
    //     if items.len() > 0  {
    //         if let CellMetaVariant::Multystate(item) = items[0] {
    //             return Some(item.clone());
    //         }
    //         return None;
    //     }
    //     // result
    //     None
    // }    

    pub fn set_label(&mut self, label: IString) {
        self.label = label;
    }

    pub fn get_cell_type(&self) -> HashSet<CellType> {
        self.types.iter()
            .map(|o| match *o {
                CellMetaVariant::Label(_) => CellType::LABEL,
                CellMetaVariant::Multystate(_) => CellType::MULTYSTATE,
                CellMetaVariant::WidgetContainer(_) => CellType::WIDGETCONTAINER,
            })
            .collect::<HashSet<_>>()
    }

    fn get_meta_position(&self, cell_type: CellType) -> Option<usize> {
        self.types.iter()
            .position(|o| {
                match cell_type {
                    CellType::LABEL => if let CellMetaVariant::Label(_) = *o { return true; },
                    CellType::MULTYSTATE => if let CellMetaVariant::Multystate(_) = *o { return true; },
                    CellType::WIDGETCONTAINER => if let CellMetaVariant::WidgetContainer(_) = *o { return true; },
                };
                false
            })
    }

    pub fn set_label_meta(&mut self, value: LabelValueXml) {
        let position = self.get_meta_position(CellType::LABEL);
        if position.is_some()  {
            let _ = std::mem::replace(&mut self.types[position.unwrap()], CellMetaVariant::Label(value));
        }
    }

    pub fn set_multystate_meta(&mut self, value: MultystateXml) {
        let position = self.get_meta_position(CellType::MULTYSTATE);
        if position.is_some()  {
            let _ = std::mem::replace(&mut self.types[position.unwrap()], CellMetaVariant::Multystate(value));
        }
    }

    pub fn get_multystate_meta(&self) -> Result<MultystateXml, JsValue>{
        let position = self.get_meta_position(CellType::MULTYSTATE);
        if position.is_some()  {
            let item = self.types[position.unwrap()].get_multystate().unwrap();
            return Ok(item);
        }
        Err(CellStateError::NotMultystate.into())
    }  

    pub fn set_widget_container_meta(&mut self, value: WidgetContainerXml) {
        let position = self.get_meta_position(CellType::WIDGETCONTAINER);
        if position.is_some()  {
            let _ = std::mem::replace(&mut self.types[position.unwrap()], CellMetaVariant::WidgetContainer(value));
        }
    }

    // pub fn get_mut_multystate(&mut self) -> Result<&mut MultystateXml, JsValue>{
    //     let item = 
    //     if let CellMetaVariant::Multystate(m) = &mut self.data {
    //         return Ok(m);
    //     }
    //     Err(CellStateError::NotMultystate.into())
    // }

  

    pub fn create_state(&mut self) {
        let position = self.get_meta_position(CellType::MULTYSTATE);
        if position.is_some() {
            let multystate = &mut self.types[position.unwrap()];
            multystate.create_state();
        }
    }
}

impl Default for CellMeta {
    fn default() -> Self {
        Self { 
            label: Default::default(), 
            types: vec![],
        }
    }
}


// /// reducer's Action
// pub enum Action {
//     SetWidgetMeta(WidgetXml),
// }

// impl Reducible for CellMeta {
//     type Action = Action;
    
//     fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
//         let position = self.get_meta_position(CellType::WIDGETCONTAINER)
//         match action {
//             Action::SetWidgetMeta(meta) => Self {
//                 label: self.label.clone(),
//                 data: CellMetaVariant::WidgetContainer(meta),
//             }.into(),
//         }
//     }
// }

// ==========================================================
#[cfg(test)]
mod tests {
    use common_model::{data_source::DataSourceXml, multystate::{state::StateXml, state_predef::StatePredefXml}, widget::WidgetContainerXml};
    use quick_xml::{de::from_str, se::to_string};

    use super::*;

    #[test]
    fn xml_cell_meta_serde_default_works() {
        let item = CellMeta::default();

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<CellMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }

    #[test]
    fn xml_cell_meta_serde_widget_works() {
        let widget = WidgetContainerXml {
            uuid: "some-uuid".into(),
            ..Default::default()
        };

        let item = CellMeta {
            label: "widget".into(),
            types: vec![CellMetaVariant::WidgetContainer(widget.into())],
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<CellMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }

    #[test]
    fn xml_cell_meta_serde_multystate_works() {
        let multy  = MultystateXml {
            range_type: Default::default(),
            states: vec![
                StateXml { pk: 0, ..Default::default() },
                StateXml { pk: 1, ..Default::default() },
            ],
            ds: Default::default(),
            predef: vec![StatePredefXml::Default(Default::default()), StatePredefXml::Bad(Default::default())],
            // bad: Default::default()
        };

        // let multy: Rc<RefCell<MultystateXml>> = Rc::new(RefCell::new(multy));

        let item = CellMeta {
            label: "multy".into(),
            types: vec![CellMetaVariant::Multystate(multy)],
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<CellMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }

    #[test]
    fn xml_cell_meta_serde_value_works() {
        let value =  LabelValueXml { ds: DataSourceXml { tag: "some_tag".into(), ..Default::default()} };

        let item = CellMeta {
            label: "value".into(),
            types: vec![CellMetaVariant::Label(value)],
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<CellMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    

    #[test]
    fn get_cell_type_works() {
        let label_meta =  LabelValueXml { ds: Default::default() } ;

        let meta = CellMeta { 
            label: Default::default(), 
            types: vec![CellMetaVariant::Label(label_meta)],
        };

        let tst = meta.get_cell_type();

        assert!(tst.contains(&CellType::LABEL));
        assert!(!tst.contains(&CellType::MULTYSTATE));
    }

    #[test]
    fn set_label_meta_works() {
        let label_meta = LabelValueXml { ds: Default::default() } ;

        let mut meta = CellMeta { 
            label: Default::default(), 
            types: vec![CellMetaVariant::Label(label_meta)],
        };

        let str = to_string(&meta).unwrap();
        println!("{str:?}");

        let from = from_str(&str).unwrap();
        println!("{from:#?}");
        assert_eq!(meta, from);        

        let new_label_meta = LabelValueXml { ds: DataSourceXml { tag: "tag-1".into(), path: "".into() } };
        meta.set_label_meta(new_label_meta);


        let str = to_string(&meta).unwrap();
        println!("{str:?}");     

        let from = from_str(&str).unwrap();
        println!("{from:#?}");
        assert_eq!(meta, from);             
        
    }

    #[test]
    fn create_state_works() {
        let multy_meta = MultystateXml { 
            range_type: Default::default(), 
            ds: Default::default(), 
            predef: Default::default(), 
            states: vec![],
        };

        let mut meta = CellMeta { 
            label: Default::default(), 
            types: vec![CellMetaVariant::Multystate(multy_meta)],
        };

        meta.create_state();

        let str = to_string(&meta).unwrap();
        println!("{str:?}");

        assert!(str.contains("<state pk=\"0\" name=\"state\""));
    }


}
