use multystate::{is_none_multystate, MultystateMeta};
use serde::{Deserialize, Serialize, Serializer};
use wasm_bindgen::JsValue;
use widget::{is_none_widget, WidgetMeta};

use super::scada_diagram::meta;

pub mod multystate;
pub mod multystate_state;
pub mod widget;

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

// impl Serialize for CellType {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match self {
//             // CellType::Undef() => "".to_owned().serialize(serializer),
//             CellType::Widget(widget) => {

//                 #[derive(Serialize, PartialEq, Debug, Clone)]
//                 struct Root {
//                     widget: WidgetMeta,
//                 }

//                 let root = Root {
//                     widget: (*widget).clone(),
//                 };

//                 root.serialize(serializer)
//             },
//             CellType::MultyState(meta) => meta.serialize(serializer),
//         }
//     }
// }

// impl<'de> Deserialize<'de> for CellType {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>
//     {
// #[derive(Debug, Deserialize)]
// struct Mapping {
//     field: i32,
//     #[serde(rename = "A")]
//     a: Option<i32>,
//     #[serde(rename = "B")]
//     b: Option<i32>,
// }

// let Mapping { field, a, b } = Mapping::deserialize(deserializer)?;

// match (a, b) {
//     (Some(_), Some(_)) =>
//         Err(D::Error::custom("multiple variants specified")),
//     (Some(a), None) =>
//         Ok(Example { field, an_enum: AnEnum::A(a) }),
//     (None, Some(b)) =>
//         Ok(Example { field, an_enum: AnEnum::B(b) }),
//     (None, None) =>
//         Err(D::Error::custom("no variants specified")),
// }
//     }
// }

fn err_not_multystate() -> JsValue {
    JsValue::from_str("this is no multistate")
} 

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "iiot")]
pub struct CellMeta {
    #[serde(rename = "@label")]
    pub label: String,
    // #[serde(rename="$value")]
    // #[serde(flatten)]
    // #[serde(serialize_with = "serialize_meta")]
    // meta: CellType,
    #[serde(skip_serializing_if = "is_none_widget")]
    pub widget: Option<WidgetMeta>,
    #[serde(skip_serializing_if = "is_none_multystate")]
    pub multystate: Option<MultystateMeta>,
}

impl CellMeta {
    pub fn set_label(&mut self, label: String) {
        self.label = label;
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
        Err(err_not_multystate())
    }

    pub fn get_multystate(&self) -> Result<&MultystateMeta, JsValue>{
        if let Some(m) = self.multystate.as_ref() {
            return Ok(m);
        }
        Err(err_not_multystate())
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
        }
    }
}

// ==========================================================
#[cfg(test)]
mod tests {
    // use multystate::StateMeta;
    use quick_xml::{de::from_str, se::to_string};
    use serde::{ser::SerializeTupleVariant, Deserializer, Serializer};

    use super::*;

    #[test]
    fn xml_cell_meta_serde_all_none_works() {
        let item = CellMeta {
            label: "test".to_owned(),
            widget: None,
            multystate: None,
            // meta: CellType::Widget(WidgetMeta {
            //     uuid: "aaaa".to_owned(),
            // }),
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
            label: "test".to_owned(),
            widget: Some(WidgetMeta {
                uuid: "some-uuid".to_owned(),
            }),
            multystate: None,
            // meta: CellType::Widget(WidgetMeta {
            //     uuid: "aaaa".to_owned(),
            // }),
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<CellMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }

    // #[test]
    // fn xml_cell_meta_serde_multystate_works() {
    //     let item = CellMeta {
    //         label: "test".to_owned(),
    //         widget: None,
    //         multystate: Some(Box::new(MultystateMeta {
    //             range_type: Default::default(),
    //             states: vec![
    //                 StateMeta { uuid: "1".to_owned() },
    //                 StateMeta { uuid: "2".to_owned() },
    //             ],
    //         })),
    //     };

    //     let str = to_string(&item).unwrap();
    //     println!("{str}");

    //     let meta = from_str::<CellMeta>(&str).unwrap();
    //     println!("{meta:#?}");

    //     assert_eq!(item, meta);
    // }

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
