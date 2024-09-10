use serde::{ser::Serializer, Deserialize, Deserializer, Serialize};

use super::multystate_state::StateMeta;

pub fn is_none_multystate(tst: &Option<MultystateMeta>) -> bool {
    match tst {
        Some(_) => false,
        None => true,
    }
}

fn unwrap_states<'de, D>(deserializer: D) -> Result<Vec<StateMeta>, D::Error>
where
    D: Deserializer<'de>,
{
    /// Represents <states>...</states>
    #[derive(Deserialize, Debug, PartialEq)]
    struct List {
        #[serde(default)]
        state: Vec<StateMeta>,
    }

    Ok(List::deserialize(deserializer)?.state)
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all="lowercase")]
pub enum RangeType {
    DISCRET,
    LINIER,
}

impl Default for RangeType {
    fn default() -> Self {
        RangeType::DISCRET
    }
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "multystate")]
pub struct MultystateMeta {
    #[serde(rename="@range-type", default)]
    pub range_type: RangeType,
    #[serde(deserialize_with = "unwrap_states", default)]
    pub states: Vec<StateMeta>,
}

impl MultystateMeta {
    
    pub fn create_state(&mut self) {
        self.states.push(StateMeta {id: "new sate".to_owned()});
    }
}

impl Default for MultystateMeta {
    fn default() -> Self {
        Self { 
            range_type: Default::default(), 
            states: Default::default() 
        }
    }
}

impl Serialize for MultystateMeta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        #[serde(rename = "multystate")]
        struct Root<'a> {
            #[serde(rename="@range-type", default)]
            range_type: &'a RangeType,
            states: List<'a>,
        }

        #[derive(Serialize)]
        struct List<'a> {
            state: &'a Vec<StateMeta>,
        }

        let helper = Root {
            range_type: &self.range_type,
            states: List {
                state: &self.states,
            },
        };
        helper.serialize(serializer)
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
            id: "some".to_owned(),
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<StateMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }

    #[test]
    fn xml_multystate_meta_nostates_serde_works() {
        let item = MultystateMeta { states: vec![], range_type: Default::default() };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<MultystateMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }

    #[test]
    fn xml_multystate_meta_states_serde_works() {
        let item = MultystateMeta {
            range_type: RangeType::LINIER,
            states: vec![
                StateMeta {
                    id: "1".to_owned(),
                },
                StateMeta {
                    id: "2".to_owned(),
                },
            ],
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<MultystateMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }

}