use implicit_clone::unsync::IString;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "ds")]
pub struct DataSource {
    #[serde(rename="@tag")]
    pub tag: IString,
    #[serde(rename="@path")]
    pub path: IString,
}

impl Default for DataSource {
    fn default() -> Self {
        Self { 
            tag: Default::default(),
            path: Default::default(),
        }
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
    fn xml_state_meta_serde_works() {
        let item = DataSource {
            tag: "proba".into(),
            ..Default::default()
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<DataSource>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    

}