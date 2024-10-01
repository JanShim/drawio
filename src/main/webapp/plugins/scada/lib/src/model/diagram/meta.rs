use implicit_clone::unsync::IString;
use serde::{Deserialize, Serialize};

use super::NULL_UUID;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Diagram {
    #[serde(rename="@uuid")]    
    pub uuid: IString,
    #[serde(rename="@name")]    
    pub name: IString,
}

impl Default for Diagram {
    fn default() -> Self {
        Self { 
            uuid: NULL_UUID.into(),
            name: Default::default(),
        }
    }
}

// ==========================================================
#[cfg(test)]
mod tests {
    use quick_xml::de::from_str;

    use super::*;

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