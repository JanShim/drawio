
use serde::{Deserialize, Serialize};
use implicit_clone::unsync::IString;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum StatePredefJson {
    Default(PredefStateJson),
    Bad(PredefStateJson),
}

impl Default for StatePredefJson {
    fn default() -> Self {
        StatePredefJson::Default(Default::default())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(rename = "predef")]
pub struct PredefStateJson {
    // pub r#type: IString,       
    pub style: IString,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum StatePredefXml {
    Default(PredefStateXml),
    Bad(PredefStateXml),
}

impl Default for StatePredefXml {
    fn default() -> Self {
        StatePredefXml::Default( Default::default() )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(rename = "default")]
pub struct PredefStateXml {
    #[serde(rename = "@style")]
    pub style: IString,
    #[serde(skip)]
    pub selected: bool,
}

impl From<StatePredefJson> for PredefStateXml {
    fn from(value: StatePredefJson) -> Self {
        match value {
            StatePredefJson::Default(state) => PredefStateXml { style: state.style, selected: false },
            StatePredefJson::Bad(state) => PredefStateXml {  style: state.style, selected: false },
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
    fn xml_serde_works() {

        #[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
        pub struct StateXml {
            #[serde(rename="$value")]                
            pub default: StatePredefXml,
        }

        let item = StateXml { default: StatePredefXml::Default(Default::default()) };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<StateXml>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    

    #[test]
    fn json_serde_works() {

        #[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
        pub struct StateJson {
            pub defalt: StatePredefJson,
        }

        let item = StateJson {
            ..Default::default()
        };

        let str = serde_json::to_string(&item).unwrap();
        println!("{str}");

        let meta = serde_json::from_str::<StateJson>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    } 
}