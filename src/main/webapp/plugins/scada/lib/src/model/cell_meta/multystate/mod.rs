use data_source::DataSource;
use serde::{ser::Serializer, Deserialize, Deserializer, Serialize};
use state::StateMeta;

pub mod state;
pub mod data_source;
pub mod state_range;

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
    #[serde(rename="ds", default)]
    pub data_source: DataSource,
    #[serde(deserialize_with = "unwrap_states", default)]
    pub states: Vec<StateMeta>,
}

impl MultystateMeta {
    pub fn create_state(&mut self) {
        self.states.push(StateMeta {
            pk: self.states.len().to_string(), 
            ..Default::default()
        });
    }

    pub fn set_data_source(&mut self, ds: DataSource) {
        self.data_source = ds;
    }

    // pub fn set_state<'a>(&'a self, meta: &'a StateMeta) {
    //     if let Some(index) = meta.pk.parse::<usize>().ok() {
    //        let mut ietm =  self.states[index];
            
    //     }
    // }
}

impl Default for MultystateMeta {
    fn default() -> Self {
        Self { 
            range_type: Default::default(), 
            data_source: Default::default(),
            states: Default::default(),
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
            #[serde(rename="ds", default)]
            pub data_source: &'a DataSource,
            states: List<'a>,
        }

        #[derive(Serialize)]
        struct List<'a> {
            state: &'a Vec<StateMeta>,
        }

        let helper = Root {
            range_type: &self.range_type,
            data_source: &self.data_source,
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
    fn xml_multystate_meta_nostates_serde_works() {
        let item = MultystateMeta::default();

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
            data_source: DataSource { 
                tag: "tag".to_owned(), 
                path: "path".to_owned(),
            },
            states: vec![
                StateMeta {
                    pk: "1".to_owned(),
                    name: "name-1".to_owned(),
                    ..Default::default()
                },
                StateMeta {
                    pk: "2".to_owned(),
                    name: "name-1".to_owned(),
                    ..Default::default()
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