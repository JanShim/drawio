use std::collections::HashSet;
use implicit_clone::{unsync::IString, ImplicitClone};
use wasm_bindgen::JsValue;
use serde::{Deserialize, Serialize};
use common_model::{
    dflow_cell::{CellType, DFlowVariant},
    geom_value::GeomValueXml, label_value::LabelValueXml,
     multystate::MultystateXml,
     widget::WidgetContainerXml
    };
use yew::AttrValue;

use crate::errors::CellStateError;

pub mod data_source_reducers;
pub mod form;

pub const CELL_TYPE_LABEL: &str = "value";
pub const CELL_TYPE_MULTY: &str = "multy";
pub const CELL_TYPE_GEOM: &str = "geom";
pub const CELL_TYPE_WIDGET_CONTAINER: &str = "widget-container";

#[derive(Debug, PartialEq, Clone, ImplicitClone)]
pub struct TypesItem {
    pub cell_type: CellType,
    pub name: AttrValue,
    pub label: AttrValue,
    pub selected: bool,
}

pub fn get_cellmeta_types(variants: &Vec<DFlowVariant>) -> HashSet<CellType> {
    variants.iter()
        .map(|o| o.get_cell_type())
        .collect::<HashSet<_>>()
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "d-flow")]
pub struct CellMeta {
    #[serde(rename = "@label")]
    pub label: IString,

    #[serde(rename = "$value", default)]
    pub types: Vec<DFlowVariant>,
}

impl CellMeta {
    pub fn set_label(&mut self, label: IString) {
        self.label = label;
    }

    pub fn get_cell_type(&self) -> HashSet<CellType> {
        self.types.iter()
            .map(|o| match *o {
                DFlowVariant::Undefiend(_) => CellType::UNDEFIEND,
                DFlowVariant::Label(_) => CellType::LABEL,
                DFlowVariant::Multystate(_) => CellType::MULTYSTATE,
                DFlowVariant::WidgetContainer(_) => CellType::WIDGETCONTAINER,
                DFlowVariant::Geometry(_) => CellType::GEOM,
            })
            .collect::<HashSet<_>>()
    }

    fn get_meta_position(&self, cell_type: CellType) -> Option<usize> {
        self.types.iter()
            .position(|o| {
                match cell_type {
                    CellType::UNDEFIEND => if let DFlowVariant::Undefiend(_) = *o { return true; },
                    CellType::LABEL => if let DFlowVariant::Label(_) = *o { return true; },
                    CellType::MULTYSTATE => if let DFlowVariant::Multystate(_) = *o { return true; },
                    CellType::GEOM =>  if let DFlowVariant::Geometry(_) = *o { return true; },
                    CellType::WIDGETCONTAINER => if let DFlowVariant::WidgetContainer(_) = *o {  log::debug!("FOUND!");  return true; },
                };
                false
            })
    }

    pub fn set_label_meta(&mut self, value: LabelValueXml) {
        let position = self.get_meta_position(CellType::LABEL);
        if position.is_some()  {
            let _ = std::mem::replace(&mut self.types[position.unwrap()], DFlowVariant::Label(value));
        }
    }

    pub fn get_label_meta(&self) -> Result<LabelValueXml, JsValue>{
        let position = self.get_meta_position(CellType::LABEL);
        if position.is_some()  {
            let item = self.types[position.unwrap()].get_label();
            return Ok(item);
        }
        Err(CellStateError::NotLabel.into())
    }

    pub fn set_multystate_meta(&mut self, value: MultystateXml) {
        let position = self.get_meta_position(CellType::MULTYSTATE);
        if position.is_some()  {
            let _ = std::mem::replace(&mut self.types[position.unwrap()], DFlowVariant::Multystate(value));
        }
    }

    pub fn get_multystate_meta(&self) -> Result<MultystateXml, JsValue>{
        let position = self.get_meta_position(CellType::MULTYSTATE);
        if position.is_some()  {
            let item = self.types[position.unwrap()].get_multystate();
            return Ok(item);
        }
        Err(CellStateError::NotMultystate.into())
    }

    pub fn set_geometry_meta(&mut self, value: GeomValueXml) {
        let position = self.get_meta_position(CellType::GEOM);
        if position.is_some()  {
            let _ = std::mem::replace(&mut self.types[position.unwrap()], DFlowVariant::Geometry(value));
        }
    }

    pub fn get_geometry_meta(&self) -> Result<GeomValueXml, JsValue>{
        let position = self.get_meta_position(CellType::GEOM);
        if position.is_some()  {
            let item = self.types[position.unwrap()].get_geometry();
            return Ok(item);
        }
        Err(CellStateError::NotGeometry.into())
    }

    pub fn set_widget_container_meta(&mut self, value: WidgetContainerXml) {
        let position = self.get_meta_position(CellType::WIDGETCONTAINER);
        if position.is_some()  {
            let _ = std::mem::replace(&mut self.types[position.unwrap()], DFlowVariant::WidgetContainer(value));
        } else {
            self.types.insert(0, DFlowVariant::WidgetContainer(value));
        }
    }

    pub fn get_widget_container_meta(&self) -> Result<WidgetContainerXml, JsValue>{
        let position = self.get_meta_position(CellType::WIDGETCONTAINER);
        if position.is_some()  {
            let item = self.types[position.unwrap()].get_widget_container();
            return Ok(item);
        }
        Err(CellStateError::NotWidgetContainer.into())
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
            types: vec![DFlowVariant::WidgetContainer(widget.into())],
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
            types: vec![DFlowVariant::Multystate(multy)],
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
            types: vec![DFlowVariant::Label(value)],
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
            types: vec![DFlowVariant::Label(label_meta)],
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
            types: vec![DFlowVariant::Label(label_meta)],
        };

        let str = to_string(&meta).unwrap();
        println!("{str:?}");

        let from = from_str(&str).unwrap();
        println!("{from:#?}");
        assert_eq!(meta, from);

        let new_label_meta = LabelValueXml { ds: DataSourceXml { tag: "tag-1".into(), path: "".into(), property: None } };
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
            types: vec![DFlowVariant::Multystate(multy_meta)],
        };

        meta.create_state();

        let str = to_string(&meta).unwrap();
        println!("{str:?}");

        assert!(str.contains("<state pk=\"0\" name=\"state\""));
    }


}
