
use serde::{Deserialize, Serialize};
use implicit_clone::unsync::IString;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(rename = "default")]
pub struct DefaultStateJson {
    pub style: IString,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(rename = "default")]
pub struct DefaultStateXml {
    #[serde(rename = "@style")]
    pub style: IString,
    #[serde(skip)]
    pub selected: bool,
}

impl From<DefaultStateJson> for DefaultStateXml {
    fn from(DefaultStateJson { style}: DefaultStateJson) -> Self {
        Self { style, selected: false }
    }
}

// ==========================================================
#[cfg(test)]
mod tests {
    use quick_xml::{
        de::from_str,
        se::to_string,
    };

    use super::*;

    #[test]
    fn xml_serde_works() {
        let item = DefaultStateXml {
            ..Default::default()
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<DefaultStateXml>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    

    #[test]
    fn json_serde_works() {
        let item = DefaultStateJson {
            ..Default::default()
        };

        let str = serde_json::to_string(&item).unwrap();
        println!("{str}");

        let meta = serde_json::from_str::<DefaultStateJson>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    } 
}