use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "state")]
pub struct StateMeta {
    #[serde(rename = "@pk")]
    pub pk: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@style")]
    pub style: String,
    #[serde(skip)]
    pub selected: bool,
}

impl Default for StateMeta {
    fn default() -> Self {
        Self { 
            pk: Default::default(),
            name: "наименование".to_owned(),
            style: "".to_owned(),
            selected: false,
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
        let item = StateMeta {
            pk: "some".to_owned(),
            ..Default::default()
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<StateMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    

}