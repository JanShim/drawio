use serde::{Deserialize, Serialize};
use web_sys::{Element, Node};
use yew::AttrValue;

use crate::schema_app::mx_cell::MxCell;

const NULL_UUID: &str = "00000000-0000-0000-0000-000000000000";

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Diagram {
    #[serde(rename(serialize="@type", deserialize="type"))]    
    pub item_type: String,
    #[serde(rename(serialize="@uuid"))]    
    pub uuid: String,
    #[serde(rename(serialize="@name"))]    
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

#[derive(Debug, PartialEq, Clone)]
pub struct ADiagram {
    pub item_type: AttrValue,
    pub uuid: AttrValue,
    pub name: AttrValue,
}

impl From<Diagram> for ADiagram {
    fn from(value: Diagram) -> Self {
        let Diagram {item_type, uuid, name} = value;
        Self {
            item_type: item_type.into(),
            uuid: uuid.into(),
            name: name.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
#[serde(rename = "object")]
pub struct Meta {
    #[serde(rename(serialize="@label"))]    
    pub label: String,
    pub diagram: Diagram, 
}



impl From<MxCell> for Meta {
    fn from(cell: MxCell) -> Self {
        if let Ok(meta) = cell.get_diagram_meta() {
            return meta;
        }
        Default::default()
    }
}

impl From<Element> for Meta {
    fn from(e: Element) -> Self {
        log::debug!("outer html: {}", e.outer_html());
        if let Ok(meta) = serde_xml_rs::from_str::<Meta>(e.outer_html().as_str()) {
            return meta;
        }  
        Default::default()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AMeta {
    pub label: AttrValue,
    pub diagram: ADiagram, 
}

impl From<Meta> for AMeta {
    fn from(value: Meta) -> Self {
        let Meta {label, diagram} = value;
        Self {
            label: label.into(),
            diagram: diagram.into(),
        }
    }
}


// ==========================================================
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn xml_diagram_meta_deser_works() {
        let xml = r#"<object label="" id="0">
      <diagram type="schema" uuid="aaaaaaaaaa" name="test"/>
    </object>"#;

        let diagram = serde_xml_rs::from_str::<Meta>(xml);    
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
        let item = Meta {
            label: "".to_owned(),
            diagram: Diagram {
                item_type: "schema".to_owned(),
                uuid: "aaaaaaaaaa".to_owned(),
                name: "test".to_owned(),
            }
        };

        let str = serde_xml_rs::to_string(&item).unwrap();
        println!("{str}");        

        let diagram = serde_xml_rs::from_str::<Meta>(&str).unwrap();    

        assert_eq!(item, diagram);
    }

    #[test]
    fn xml_diagram_deser_works() {
        let xml = r#"<diagram type="schema" uuid="aaaaaaaaaa" />"#;

        let diagram = serde_xml_rs::from_str::<Diagram>(xml);    
        match diagram {
            Ok(item) => {
                assert_eq!(item.item_type, "schema".to_owned());
                assert_eq!(item.uuid, "aaaaaaaaaa".to_owned());
            },
            Err(err) => panic!("err: {}", err),
        }
    }    


}