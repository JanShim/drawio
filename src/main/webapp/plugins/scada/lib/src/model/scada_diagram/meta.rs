use serde::{Deserialize, Serialize};
use quick_xml::{de::from_str, se::to_string};
use web_sys::Element;

use crate::model::mx_cell::MxCell;

use super::NULL_UUID;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Diagram {
    #[serde(rename="@type")]    
    pub item_type: String,
    #[serde(rename="@uuid")]    
    pub uuid: String,
    #[serde(rename="@name")]    
    pub name: String,
}

impl Default for Diagram {
    fn default() -> Self {
        Self { 
            item_type: "schema".to_owned(),
            uuid: NULL_UUID.to_owned(),
            name: "undefiend".to_owned(),
        }
    }
}

// #[derive(Debug, PartialEq, Clone)]
// pub struct ADiagram {
//     pub item_type: AttrValue,
//     pub uuid: AttrValue,
//     pub name: AttrValue,
// }

// impl From<Diagram> for ADiagram {
//     fn from(value: Diagram) -> Self {
//         let Diagram {item_type, uuid, name} = value;
//         Self {
//             item_type: item_type.into(),
//             uuid: uuid.into(),
//             name: name.into(),
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
#[serde(rename = "object")]
pub struct DiagramMeta {
    #[serde(rename="@label")]    
    pub label: String,
    pub diagram: Diagram, 
}



impl From<MxCell> for DiagramMeta {
    fn from(cell: MxCell) -> Self {
        if let Ok(meta) = cell.get_diagram_meta() {
            return meta;
        }
        Default::default()
    }
}

impl From<Element> for DiagramMeta {
    fn from(e: Element) -> Self {
        log::debug!("outer html: {}", e.outer_html());
        if let Ok(meta) = from_str::<DiagramMeta>(e.outer_html().as_str()) {
            return meta;
        }  
        Default::default()
    }
}

// #[derive(Debug, PartialEq, Clone)]
// pub struct AMeta {
//     pub label: AttrValue,
//     pub diagram: ADiagram, 
// }

// impl From<Meta> for AMeta {
//     fn from(value: Meta) -> Self {
//         let Meta {label, diagram} = value;
//         Self {
//             label: label.into(),
//             diagram: diagram.into(),
//         }
//     }
// }


// ==========================================================
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn xml_diagram_meta_deser_works() {
        let xml = r#"<object label="" id="0">
      <diagram type="schema" uuid="aaaaaaaaaa" name="test"/>
    </object>"#;

        let diagram = from_str::<DiagramMeta>(xml);    
        match diagram {
            Ok(item) => {
                println!("{item:#?}");
                // assert_eq!(item.id, "0".to_owned());
                assert_eq!(item.diagram.uuid, "aaaaaaaaaa".to_owned());

            },
            Err(err) => panic!("err: {}", err),
        }
    }

    #[test]
    fn xml_diagram_meta_ser_works() {
        let item = DiagramMeta {
            label: "".to_owned(),
            diagram: Diagram {
                item_type: "schema".to_owned(),
                uuid: "aaaaaaaaaa".to_owned(),
                name: "test".to_owned(),
            }
        };

        let str = to_string(&item).unwrap();
        println!("{str}");        

        let diagram = from_str::<DiagramMeta>(&str).unwrap();    

        assert_eq!(item, diagram);
    }

    #[test]
    fn xml_diagram_deser_works() {
        let xml = r#"<diagram type="schema" uuid="aaaaaaaaaa" name="test"/>"#;

        let diagram = from_str::<Diagram>(xml);    
        match diagram {
            Ok(item) => {
                assert_eq!(item.item_type, "schema".to_owned());
                assert_eq!(item.uuid, "aaaaaaaaaa".to_owned());
                assert_eq!(item.name, "test".to_owned());
            },
            Err(err) => panic!("err: {}", err),
        }
    }    


}