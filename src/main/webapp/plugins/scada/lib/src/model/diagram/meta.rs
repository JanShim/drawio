use implicit_clone::{unsync::IString, ImplicitClone};
use serde::{Deserialize, Serialize};
use web_sys::FormData;

use super::NULL_UUID;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Diagram {
    // #[serde(rename="@uuid")]    
    // pub uuid: IString,
    // #[serde(rename="@name")]    
    // pub name: IString,
}

impl Default for Diagram {
    fn default() -> Self {
        Self { 
            // uuid: NULL_UUID.into(),
            // name: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, ImplicitClone)]
pub struct DiagramForm {
    pub uuid: IString,
    pub name: IString,
}

impl DiagramForm {
    pub fn is_new_item(&self) -> bool {
        self.uuid == NULL_UUID
    }
}

impl Default for DiagramForm {
    fn default() -> Self {
        Self { 
            uuid: NULL_UUID.into(), 
            name: Default::default() 
        }
    }
}

impl From<FormData> for DiagramForm {
    fn from(data: FormData) -> Self {
        Self { 
            uuid: data.get("uuid").as_string().unwrap_or_default().into(), 
            name: data.get("name").as_string().unwrap_or_default().into(), 
        }
    }
}

// ==========================================================
#[cfg(test)]
mod tests {
    use quick_xml::{de::from_str, se::to_string};

    use super::*;

    #[test]
    fn xml_deser_works() {
        let item = Diagram::default();

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<Diagram>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }   

}