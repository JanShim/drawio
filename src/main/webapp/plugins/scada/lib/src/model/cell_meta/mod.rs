use std::collections::HashSet;
use common_model::{geom_value::GeomValueXml, label_value::LabelValueXml, multystate::MultystateXml, undefiend::UndefiendXml, widget::WidgetContainerXml};
use implicit_clone::unsync::IString;
use wasm_bindgen::JsValue;
use web_sys::FormData;
use serde::{Deserialize, Serialize};

use crate::errors::CellStateError;

pub mod data_source_reducers;
pub mod widget_reducers;
pub mod value_reducers;

pub const CELL_TYPE_LABEL: &str = "value";
pub const CELL_TYPE_MULTY: &str = "multy";
pub const CELL_TYPE_GEOM: &str = "geom";


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CellType {
    UNDEFIEND,
    LABEL,
    MULTYSTATE,
    WIDGETCONTAINER,
    GEOM,
}

// impl From<FormData> for CellType {
//     fn from(data: FormData) -> Self {
//         match data.get("cell-type").as_string() {
//             Some(value) => match value {
//                 _ if value==CELL_TYPE_LABEL => CellType::LABEL,
//                 _ if value==CELL_TYPE_GEOM => CellType::GEOM,
//                 _ => CellType::MULTYSTATE,
//             },
//             None => CellType::MULTYSTATE,
//         }
//     }
// }


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum CellMetaVariant {
    #[serde(rename = "undefiend")]
    Undefiend(UndefiendXml),
    #[serde(rename = "label")]
    Label(LabelValueXml),
    #[serde(rename = "multystate")]
    Multystate(MultystateXml),
    #[serde(rename = "widget-container")]
    WidgetContainer(WidgetContainerXml),
    #[serde(rename = "geometry")]
    Geometry(GeomValueXml),
}

impl CellMetaVariant {
    pub fn get_cell_type(&self) -> CellType {
        match self {
            CellMetaVariant::Undefiend(_) => CellType::UNDEFIEND,
            CellMetaVariant::Label(_) => CellType::LABEL,
            CellMetaVariant::Multystate(_) => CellType::MULTYSTATE,
            CellMetaVariant::WidgetContainer(_) => CellType::WIDGETCONTAINER,
            CellMetaVariant::Geometry(_) => CellType::GEOM,
        }
    }

    pub fn create_state(&mut self) {
        if let Self::Multystate(multy) = self {
            multy.create_state();
        }
    }

    pub fn get_label(&self) -> Option<LabelValueXml> {
        match self {
            CellMetaVariant::Label(vaue) => Some(vaue.clone()),
            _ => None
        }
    }

    pub fn get_multystate(&self) -> Option<MultystateXml> {
        match self {
            CellMetaVariant::Multystate(value) => Some(value.clone()),
            _ => None
        }
    }

    pub fn get_widget_container(&self) -> Option<WidgetContainerXml> {
        match self {
            CellMetaVariant::WidgetContainer(value) => Some(value.clone()),
            _ => None
        }
    }

    pub fn get_geometry(&self) -> Option<GeomValueXml> {
        match self {
            CellMetaVariant::Geometry(value) => Some(value.clone()),
            _ => None
        }
    }    
}


pub fn get_cellmeta_types(variants: &Vec<CellMetaVariant>) -> HashSet<CellType> {
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
    pub types: Vec<CellMetaVariant>,
}

impl CellMeta {
    pub fn set_label(&mut self, label: IString) {
        self.label = label;
    }

    pub fn get_cell_type(&self) -> HashSet<CellType> {
        self.types.iter()
            .map(|o| match *o {
                CellMetaVariant::Undefiend(_) => CellType::UNDEFIEND,
                CellMetaVariant::Label(_) => CellType::LABEL,
                CellMetaVariant::Multystate(_) => CellType::MULTYSTATE,
                CellMetaVariant::WidgetContainer(_) => CellType::WIDGETCONTAINER,
                CellMetaVariant::Geometry(_) => CellType::GEOM,
            })
            .collect::<HashSet<_>>()
    }

    fn get_meta_position(&self, cell_type: CellType) -> Option<usize> {
        self.types.iter()
            .position(|o| {
                match cell_type {
                    CellType::UNDEFIEND => if let CellMetaVariant::Undefiend(_) = *o { return true; },
                    CellType::LABEL => if let CellMetaVariant::Label(_) = *o { return true; },
                    CellType::MULTYSTATE => if let CellMetaVariant::Multystate(_) = *o { return true; },
                    CellType::GEOM =>  if let CellMetaVariant::Geometry(_) = *o { return true; },
                    CellType::WIDGETCONTAINER => if let CellMetaVariant::WidgetContainer(_) = *o {  log::debug!("FOUND!");  return true; },
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

    pub fn get_label_meta(&self) -> Result<LabelValueXml, JsValue>{
        let position = self.get_meta_position(CellType::LABEL);
        if position.is_some()  {
            let item = self.types[position.unwrap()].get_label().unwrap();
            return Ok(item);
        }
        Err(CellStateError::NotLabel.into())
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

    pub fn set_geometry_meta(&mut self, value: GeomValueXml) {
        let position = self.get_meta_position(CellType::GEOM);
        if position.is_some()  {
            let _ = std::mem::replace(&mut self.types[position.unwrap()], CellMetaVariant::Geometry(value));
        }
    }    

    pub fn get_geometry_meta(&self) -> Result<GeomValueXml, JsValue>{
        let position = self.get_meta_position(CellType::GEOM);
        if position.is_some()  {
            let item = self.types[position.unwrap()].get_geometry().unwrap();
            return Ok(item);
        }
        Err(CellStateError::NotGeometry.into())
    }      

    pub fn set_widget_container_meta(&mut self, value: WidgetContainerXml) {
        let position = self.get_meta_position(CellType::WIDGETCONTAINER);
        if position.is_some()  {
            let _ = std::mem::replace(&mut self.types[position.unwrap()], CellMetaVariant::WidgetContainer(value));
        } else {
            self.types.insert(0, CellMetaVariant::WidgetContainer(value));
        }
    }

    pub fn get_widget_container_meta(&self) -> Result<WidgetContainerXml, JsValue>{
        let position = self.get_meta_position(CellType::WIDGETCONTAINER);
        if position.is_some()  {
            let item = self.types[position.unwrap()].get_widget_container().unwrap();
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
