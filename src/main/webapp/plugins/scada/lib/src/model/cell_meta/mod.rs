
use serde::{Deserialize, Serialize};
use widget::{WidgetMeta, is_none_widget };
use multystate::{MultystateMeta, is_none_multystate};


pub mod widget;
pub mod multystate;

pub enum CellType {
    Widget(WidgetMeta)
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "iiot")]
pub struct CellMeta {
    #[serde(rename="@label")]
    pub label: String,
    #[serde(skip_serializing_if = "is_none_widget")]
    pub widget: Option<WidgetMeta>,
    #[serde(skip_serializing_if = "is_none_multystate")]
    pub multystate: Option<MultystateMeta>,

}

// ==========================================================
#[cfg(test)]
mod tests {
    use multystate::StateMeta;
    use quick_xml::{
        de::from_str,
        se::to_string,
    };
    use serde::{Deserializer, Serializer};

    use super::*;

    #[test]
    fn xml_cell_meta_serde_all_none_works() {
        let item = CellMeta {
            label: "test".to_owned(),
            widget: None,
            multystate: None,
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
            label: "test".to_owned(),
            widget: None,
            multystate: Some(MultystateMeta {
                range_type: Default::default(),
                states: vec![
                    StateMeta { uuid: "1".to_owned() },
                    StateMeta { uuid: "2".to_owned() },
                ],
            }),
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<CellMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }   


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
