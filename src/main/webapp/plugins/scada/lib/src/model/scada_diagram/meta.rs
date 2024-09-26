use serde::{Deserialize, Serialize};
use quick_xml::{de::from_str, se::to_string};
use web_sys::Element;

use crate::model::mx_cell::MxCell;

use super::NULL_UUID;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Diagram {
    #[serde(rename="@uuid")]    
    pub uuid: String,
    #[serde(rename="@name")]    
    pub name: String,
}

impl Default for Diagram {
    fn default() -> Self {
        Self { 
            uuid: NULL_UUID.to_owned(),
            name: Default::default(),
        }
    }
}

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
        if let Ok(meta) = from_str::<DiagramMeta>(e.outer_html().as_str()) {
            return meta;
        }  
        log::error!("can't create diagram meta form: {}", e.outer_html());
        Default::default()
    }
}

// ==========================================================
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn xml_diagram_meta_deser_works() {
        let xml = r#"<object label="" id="0">
      <diagram uuid="aaaaaaaaaa" name="test"/>
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
        let xml = r#"<diagram uuid="aaaaaaaaaa" name="test"/>"#;

        let diagram = from_str::<Diagram>(xml);    
        match diagram {
            Ok(item) => {
                assert_eq!(item.uuid, "aaaaaaaaaa");
                assert_eq!(item.name, "test");
            },
            Err(err) => panic!("err: {}", err),
        }
    }    


}