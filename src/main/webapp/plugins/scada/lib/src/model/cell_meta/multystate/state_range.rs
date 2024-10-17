use std::fmt;
use implicit_clone::ImplicitClone;
use serde::{ Deserialize, Serialize};


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ImplicitClone)]
#[serde(rename_all="lowercase")]
pub enum RangeType {
    DISCRET,
    RANGE,
}

impl Default for RangeType {
    fn default() -> Self {
        RangeType::DISCRET
    }
}

impl From<RangeValue> for RangeType {    
    fn from(value: RangeValue) -> Self {
        match value {
            RangeValue::DiscretConst { value:_ } => RangeType::DISCRET,
            RangeValue::RangeConst { from:_, to:_ } => RangeType::RANGE,
            RangeValue::DiscretTag { value:_ } => RangeType::DISCRET,
            RangeValue::RangeTag { from:_, to:_ } => RangeType::RANGE,
        }
    }
}

impl fmt::Display for RangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RangeType::DISCRET => write!(f, "discret"),
            RangeType::RANGE => write!(f, "linear"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum RangeValueJson {
    DiscretConst { value: u32 },
    RangeConst { from: f32, to: f32 },     //  value in (from, to]
    DiscretTag { value: String },
    RangeTag { from: String, to: String },     //  value in (from, to]    
}

impl Default for RangeValueJson {
    fn default() -> Self {
        RangeValueJson::DiscretConst {value: 0 }
    }
}

impl From<RangeValue> for RangeValueJson  {
    fn from(value: RangeValue) -> Self {
        match value {
            RangeValue::DiscretConst { value } => RangeValueJson::DiscretConst { value },
            RangeValue::DiscretTag { value } => RangeValueJson::DiscretTag { value },
            RangeValue::RangeConst { from, to } => RangeValueJson::RangeConst { from, to },
            RangeValue::RangeTag { from, to } => RangeValueJson::RangeTag { from, to },
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum RangeValue {
    DiscretConst { 
         #[serde(rename = "@value")]
        value: u32,
    },
    DiscretTag { 
        #[serde(rename = "@value")]
       value: String,
    },    
    RangeConst { 
        #[serde(rename = "@from")]
        from: f32, 
        #[serde(rename = "@to")]
        to: f32,
    },
    RangeTag { 
        #[serde(rename = "@from")]
        from: String, 
        #[serde(rename = "@to")]
        to: String,
    },
}

impl RangeValue {
    pub fn get_value(&self) -> u32 {
        match self {
            RangeValue::DiscretConst { value } => *value,
            _ => 0,
        }
    }
    pub fn get_to(&self) -> f32 {
        match self {
            RangeValue::RangeConst { from:_, to  } => *to,
            _ => f32::MIN,
        }
    }    
    pub fn get_from(&self) -> f32 {
        match self {
            RangeValue::RangeConst { from, to:_  } => *from,
            _ => f32::MIN,
        }
    }        
}

impl Default for RangeValue {
    fn default() -> Self {
        RangeValue::DiscretConst {value: 0, }
    }
}

impl fmt::Display for RangeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RangeValue::DiscretConst { value } =>  write!(f, "{value}"),
            RangeValue::RangeConst { from:_, to } =>  write!(f, "{to}"),
            RangeValue::DiscretTag { value } => write!(f, "{value}"),
            RangeValue::RangeTag { from:_, to } => write!(f, "{to}"),
        }
    }
}

impl From<RangeValueJson> for  RangeValue {
    fn from(value: RangeValueJson) -> Self {
        match value {
            RangeValueJson::DiscretConst { value } => RangeValue::DiscretConst { value },
            RangeValueJson::DiscretTag { value } => RangeValue::DiscretTag { value },
            RangeValueJson::RangeConst { from, to } => RangeValue::RangeConst { from, to },
            RangeValueJson::RangeTag { from, to } => RangeValue::RangeTag { from, to },
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
    fn xml_range_discret_serde_works() {
        let item = RangeValue::default();

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<RangeValue>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    

    #[test]
    fn xml_range_discret_tag_serde_works() {
        let item = RangeValue::DiscretTag { value: "disc-tag".to_owned() };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<RangeValue>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }      

    #[test]
    fn xml_range_range_const_serde_works() {
        let item = RangeValue::RangeConst { from: 0.1, to: 0.2, };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<RangeValue>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    } 

    #[test]
    fn xml_range_range_tag_serde_works() {
        let item = RangeValue::RangeTag { from: "from".to_owned(), to: "to".to_owned() };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<RangeValue>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }        

    #[test]
    fn json_discret_constant_serde_works() {
        let item = RangeValueJson::default();

        let str = serde_json::to_string::<RangeValueJson>(&item).unwrap();
        println!("{str}");

        let meta = serde_json::from_str::<RangeValueJson>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }

    #[test]
    fn json_discret_tag_serde_works() {
        let item = RangeValueJson::DiscretTag { value: "some-tag".to_owned() };

        let str = serde_json::to_string::<RangeValueJson>(&item).unwrap();
        println!("{str}");

        let meta = serde_json::from_str::<RangeValueJson>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }       

    #[test]
    fn json_range_linear_tag_serde_works() {
        let item =  RangeValueJson::RangeTag { 
            from: "from".to_owned(), 
            to: "to".to_owned(), 
        };

        let str = serde_json::to_string::<RangeValueJson>(&item).unwrap();
        println!("{str}");

        let meta = serde_json::from_str::<RangeValueJson>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    

    #[test]
    fn json_from_works() {
        let item = RangeValue::RangeConst { from: 0.1, to: 0.2 } ;
        let item: RangeValueJson = item.into();

        let str = serde_json::to_string::<RangeValueJson>(&item).unwrap();
        println!("{str}");

        let meta = serde_json::from_str::<RangeValueJson>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }      

    #[test]
    fn json_from2_works() {
        let item =  RangeValueJson::RangeTag { 
            from: "from".to_owned(), 
            to: "to".to_owned(), 
        };
        let item: RangeValue = item.into();

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<RangeValue>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }        

}