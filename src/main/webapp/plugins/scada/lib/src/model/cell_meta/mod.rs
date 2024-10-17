use implicit_clone::unsync::IString;
use wasm_bindgen::JsValue;
use web_sys::FormData;
use yew::Reducible;
use serde::{Deserialize, Serialize};
use undefiend::UndefiendMeta;
use multystate::MultystateMeta;
use value::ValueMeta;
use widget::WidgetMeta;

use crate::errors::CellStateError;

pub mod multystate;
pub mod widget;
pub mod value;
pub mod undefiend;

#[derive(Debug)]
pub enum CellType {
    UNDEFIEND,
    WIDGET,
    MULTYSTATE,
    VALUE,
}

impl From<FormData> for CellType {
    fn from(data: FormData) -> Self {
        match data.get("cell-type").as_string() {
            Some(value) => match value {
                _ if value=="value" => CellType::VALUE,
                _ => CellType::MULTYSTATE,
            },
            None => CellType::MULTYSTATE,
        }
    }
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum CellMetaVariant {
    #[serde(rename = "undefiend")]
    Undefiend(UndefiendMeta),
    #[serde(rename = "value")]
    Value(ValueMeta),
    #[serde(rename = "multystate")]
    Multystate(MultystateMeta),
    #[serde(rename = "widget")]
    Widget(WidgetMeta),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "d-flow")]
pub struct CellMeta {
    #[serde(rename = "@label")]
    pub label: IString,

    #[serde(rename = "$value")]
    pub data: CellMetaVariant,
}


impl CellMeta {
    pub fn set_label(&mut self, label: IString) {
        self.label = label;
    }

    pub fn set_value_meta(&mut self, value: ValueMeta) {
        if let CellMetaVariant::Value(_) = self.data {
            self.data = CellMetaVariant::Value(value);
        }
    }

    pub fn get_cell_type(&self) -> CellType {
        match self.data {
            CellMetaVariant::Undefiend(_) => CellType::UNDEFIEND,
            CellMetaVariant::Value(_) => CellType::VALUE,
            CellMetaVariant::Multystate(_) => CellType::MULTYSTATE,
            CellMetaVariant::Widget(_) => CellType::WIDGET,
        }
    }

    pub fn get_mut_multystate(&mut self) -> Result<&mut MultystateMeta, JsValue>{
        if let CellMetaVariant::Multystate(m) = &mut self.data {
            return Ok(m);
        }
        Err(CellStateError::NotMultystate.into())
    }

    pub fn get_multystate(&self) -> Result<&MultystateMeta, JsValue>{
        if let CellMetaVariant::Multystate(m) = &self.data {
            return Ok(m);
        }
        Err(CellStateError::NotMultystate.into())
    }    

    pub fn create_state(&mut self) {
        if let CellMetaVariant::Multystate(multystate) = &mut self.data {
            multystate.create_state();
        }
    }
}

impl Default for CellMeta {
    fn default() -> Self {
        Self { 
            label: Default::default(), 
            data: CellMetaVariant::Undefiend(Default::default()),
        }
    }
}


/// reducer's Action
pub enum Action {
    SetWidgetMeta(WidgetMeta),
}

impl Reducible for CellMeta {
    type Action = Action;
    
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            Action::SetWidgetMeta(meta) => Self {
                label: self.label.clone(),
                data: CellMetaVariant::Widget(meta),
            }.into(),
        }
    }

}

// ==========================================================
#[cfg(test)]
mod tests {
    use multystate::state::StateMeta;
    use quick_xml::{de::from_str, se::to_string};
    use serde::{ser::SerializeTupleVariant, Deserializer, Serializer};
    use value::ValueMeta;

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
        let widget = WidgetMeta {
            uuid: "some-uuid".into(),
            ..Default::default()
        };

        let item = CellMeta {
            label: "widget".into(),
            data: CellMetaVariant::Widget(widget),
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<CellMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }

    #[test]
    fn xml_cell_meta_serde_multystate_works() {
        let multy = MultystateMeta {
            range_type: Default::default(),
            states: vec![
                StateMeta { pk: 0, ..Default::default() },
                StateMeta { pk: 1, ..Default::default() },
            ],
            data_source: Default::default(),
        };

        let item = CellMeta {
            label: "multy".into(),
            data: CellMetaVariant::Multystate(multy),
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<CellMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }

    #[test]
    fn xml_cell_meta_serde_value_works() {
        let value = ValueMeta { tag: "some_tag".into(), ..Default::default() };

        let item = CellMeta {
            label: "value".into(),
            data: CellMetaVariant::Value(value),
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<CellMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    
   

    /* #region example serialize_tuple_variant*/
    enum E {
        T(u8),
        U(String),
    }

    impl Serialize for E {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match *self {
                E::T(ref a) => {
                    let mut tv = serializer.serialize_tuple_variant("E", 0, "T", 1)?;
                    tv.serialize_field(a)?;
                    tv.end()
                }
                E::U(ref a) => {
                    let mut tv = serializer.serialize_tuple_variant("E", 1, "U", 1)?;
                    tv.serialize_field(a)?;
                    tv.end()
                }
            }
        }
    }
    

    #[test]
    fn example_serialize_tuple_variant_works() {

        #[derive(Serialize)]
        pub struct CellMeta {
            meta: E,
        }

        let meta = CellMeta {
            meta: E::T(123),
        };

        let str = to_string(&meta);
        println!("{str:#?}")

    }

    /* #endregion */

    /* #region  example for array */
    fn unwrap_list<'de, D>(deserializer: D) -> Result<Vec<Element>, D::Error>
    where
        D: Deserializer<'de>,
    {
        /// Represents <list>...</list>
        #[derive(Deserialize, Debug, PartialEq)]
        struct List {
            #[serde(default)]
            // #[serde(rename(serialize="list"))]
            element: Vec<Element>,
        }

        Ok(List::deserialize(deserializer)?.element)
    }

    /// Represents <element/>
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename = "element")]
    struct Element {
        #[serde(rename = "@attr")]
        attr: String,
    }

    #[test]
    fn xml_element_serde_works() {
        let item = Element {
            attr: "aaaa".to_owned(),
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<Element>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }

    #[derive(Deserialize, Debug, PartialEq)]
    #[serde(rename = "root")]
    struct Root {
        #[serde(deserialize_with = "unwrap_list")]
        pub list: Vec<Element>,
    }

    impl Serialize for Root {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            #[derive(Serialize)]
            #[serde(rename = "root")]
            struct Root<'a> {
                list: List<'a>,
            }

            #[derive(Serialize)]
            #[serde(rename = "list")]
            struct List<'a> {
                element: &'a Vec<Element>,
            }

            let helper = Root {
                list: List {
                    element: &self.list,
                },
            };
            helper.serialize(serializer)
        }
    }

    #[test]
    fn example_serde_works() {
        // let from = AnyName { list: List { elements: vec![(), (), ()] } };
        let root = Root {
            list: vec![
                Element {
                    attr: "1".to_owned(),
                },
                Element {
                    attr: "2".to_owned(),
                },
            ],
        };
        println!("{root:#?}");

        let str = to_string(&root).unwrap();
        println!("{str:#?}");

        let from = from_str(&str).unwrap();
        println!("{from:#?}");

        assert_eq!(root, from);
    }

    /* #endregion */
}
