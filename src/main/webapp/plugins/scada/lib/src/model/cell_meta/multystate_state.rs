use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "state")]
pub struct StateMeta {
    #[serde(rename = "@pk")]
    pub pk: String,
}

impl Default for StateMeta {
    fn default() -> Self {
        Self { pk: Default::default() }
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
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<StateMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    

}