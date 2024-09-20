use std::fmt;
use implicit_clone::ImplicitClone;
use serde::{ Deserialize, Serialize};


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ImplicitClone)]
#[serde(rename_all="lowercase")]
pub enum RangeType {
    DISCRET,
    LINEAR,
}

impl Default for RangeType {
    fn default() -> Self {
        RangeType::DISCRET
    }
}

impl From<Range> for RangeType {    
    fn from(value: Range) -> Self {
        match value {
            Range::Discret { value:_ } => RangeType::DISCRET,
            Range::Linear { from:_, to:_ } => RangeType::LINEAR,
        }
    }
}

impl fmt::Display for RangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RangeType::DISCRET => write!(f, "discret"),
            RangeType::LINEAR => write!(f, "linear"),
        }
    }
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ImplicitClone)]
pub enum Range {
    #[serde(rename = "discret")]
    Discret { 
        // #[serde(skip)]
        // prev: u32,
         #[serde(rename = "@value")]
        value: u32,
    },
    #[serde(rename = "linear")]
    Linear { 
        // #[serde(skip)]
        // prev: f32,
        #[serde(rename = "@from")]
        from: f32, 
        #[serde(rename = "@to")]
        to: f32,
    },
}

impl Range {
    pub fn get_value(&self) -> u32 {
        match self {
            Range::Discret { value } => *value,
            _ => 0,
        }
    }
    pub fn get_to(&self) -> f32 {
        match self {
            Range::Linear { from:_, to  } => *to,
            _ => f32::MIN,
        }
    }    
    pub fn get_from(&self) -> f32 {
        match self {
            Range::Linear { from, to:_  } => *from,
            _ => f32::MIN,
        }
    }        
}

impl Default for Range {
    fn default() -> Self {
        Range::Discret {value: 0, }
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Range::Discret { value } =>  write!(f, "{value}"),
            Range::Linear { from:_, to } =>  write!(f, "{to}"),
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
        let item = Range::default();

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<Range>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    

    #[test]
    fn xml_range_linear_serde_works() {
        let item = Range::Linear { from: 0.1, to: 0.2, };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<Range>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    


}