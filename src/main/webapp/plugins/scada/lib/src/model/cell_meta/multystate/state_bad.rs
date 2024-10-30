use serde::{Deserialize, Serialize};
use implicit_clone::unsync::IString;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(rename = "bad")]
pub struct BadStateJson {
    pub style: IString,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(rename = "bad")]
pub struct BadStateXml {
    #[serde(rename = "@style")]
    pub style: IString,
    #[serde(skip)]
    pub selected: bool,
}

impl From<BadStateJson> for BadStateXml {
    fn from(BadStateJson { style }: BadStateJson) -> Self {
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
        let item = BadStateXml {
            ..Default::default()
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<BadStateXml>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    

    #[test]
    fn json_serde_works() {
        let item = BadStateJson {
            ..Default::default()
        };

        let str = serde_json::to_string(&item).unwrap();
        println!("{str}");

        let meta = serde_json::from_str::<BadStateJson>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }     
}