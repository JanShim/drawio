use implicit_clone::unsync::IString;
use wasm_bindgen::JsValue;
use yew::Reducible;
use serde::{Deserialize, Serialize};
use multystate::{is_none_multystate, MultystateMeta};
use value::{is_none_value, ValueMeta};
use widget::{is_none_widget, WidgetMeta};

use crate::errors::CellStateError;

pub mod multystate;
pub mod widget;
pub mod value;

// fn serialize_meta<S>(item: &CellType, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//         match item {
//             // CellType::Undef() => "".to_owned().serialize(serializer),
//             CellType::Widget(widget) => {

//                 #[derive(Serialize, PartialEq, Debug, Clone)]
//                 struct Root {
//                     widget: WidgetMeta,
//                 }

//                 let root = Root {
//                     widget: (*widget).clone(),
//                 };

//                 root.clone().serialize(serializer)
//             },
//             CellType::MultyState(meta) => meta.serialize(serializer),
//         }
// }

// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// #[serde(untagged)]
pub enum CellType {
    UNDEFIEND,
    WIDGET,
    MULTYSTATE,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "iiot")]
pub struct CellMeta {
    #[serde(rename = "@label")]
    pub label: IString,

    #[serde(skip_serializing_if = "is_none_value")]
    pub value: Option<ValueMeta>,

    #[serde(skip_serializing_if = "is_none_widget")]
    pub widget: Option<WidgetMeta>,

    #[serde(skip_serializing_if = "is_none_multystate")]
    pub multystate: Option<MultystateMeta>,
}

impl CellMeta {
    pub fn set_label(&mut self, label: IString) {
        self.label = label;
    }

    pub fn set_value_meta(&mut self, value: ValueMeta) {
        self.value.replace(value);
    }

    pub fn get_cell_type(&self) -> CellType {
        if let Some(_) = self.widget  {
            return CellType::WIDGET;
        } else if let Some(_) = self.multystate {
            return CellType::MULTYSTATE;
        }
        CellType::UNDEFIEND
    }

    pub fn get_mut_multystate(&mut self) -> Result<&mut MultystateMeta, JsValue>{
        if let Some(m) = self.multystate.as_mut() {
            return Ok(m);
        }
        Err(CellStateError::NotMultystate.into())
    }

    pub fn get_multystate(&self) -> Result<&MultystateMeta, JsValue>{
        if let Some(m) = self.multystate.as_ref() {
            return Ok(m);
        }
        Err(CellStateError::NotMultystate.into())
    }    

    pub fn create_state(&self) {
        todo!();
        // if let Some(mut multystate) = self.multystate.clone() {
        //     multystate.as_mut().create_state();
        // }
    }
}

impl Default for CellMeta {
    fn default() -> Self {
        Self { 
            label: Default::default(), 
            widget: None, 
            multystate: None, 
            value: None,
        }
    }
}


/// reducer's Action
pub enum Action {
    // SetStyle(IString),
}

impl Reducible for CellMeta {
    type Action = Action;
    
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        // match action {
        //     // Action::SetStyle(style) => Self {
        //     //     // label: self.label.clone(),
        //     //     // widget: self.widget.clone(),
        //     //     // multystate: self.multystate.clone(),
        //     // }.into(),
        //     _ => self
        // }
        self
    }

}

// ==========================================================
#[cfg(test)]
mod tests {
    use multystate::state::StateMeta;
    // use multystate::StateMeta;
    use quick_xml::{de::from_str, se::to_string};
    use serde::{ser::SerializeTupleVariant, Deserializer, Serializer};

    use super::*;

    #[test]
    fn xml_cell_meta_serde_all_none_works() {
        let item = CellMeta {
            label: "test".into(),
            ..Default::default()
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<CellMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }

    #[test]
    fn xml_cell_meta_serde_widget_works() {
        let item = CellMeta {
            label: "test".into(),
            widget: Some(WidgetMeta {
                uuid: "some-uuid".into(),
            }),
            ..Default::default()
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<CellMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }

    #[test]
    fn xml_cell_meta_serde_multystate_works() {
        let item = CellMeta {
            label: "test".into(),
            multystate: Some(MultystateMeta {
                range_type: Default::default(),
                states: vec![
                    StateMeta { pk: 0, ..Default::default() },
                    StateMeta { pk: 1, ..Default::default() },
                ],
                data_source: Default::default(),
            }),
            ..Default::default()
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<CellMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }

    #[test]
    fn xml_cell_meta_serde_value_works() {
        let item = CellMeta {
            label: "test".into(),
            value: Some(ValueMeta { tag: "some_tag".into(), ..Default::default() }),
            ..Default::default()
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
