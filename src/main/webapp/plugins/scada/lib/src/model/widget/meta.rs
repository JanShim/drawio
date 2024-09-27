use serde::{Deserialize, Serialize};
use quick_xml::{de::from_str, se::to_string};
use web_sys::Element;

use crate::model::mx_cell::MxCell;

use super::NULL_UUID;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "widget")]
pub struct Widget {
    #[serde(rename="@uuid")]    
    pub uuid: String,
    #[serde(rename="@name")]    
    pub name: String,
    #[serde(rename="@group")]    
    pub group: String,
}

impl Default for Widget {
    fn default() -> Self {
        Self { 
            uuid: NULL_UUID.to_owned(),
            name: Default::default(),
            group: Default::default(),
        }
    }
}

// #[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
// #[serde(rename = "object")]
// pub struct WidgetMeta {
//     #[serde(rename="@label")]    
//     pub label: String,
//     pub widget: Widget, 
// }


// impl From<MxCell> for WidgetMeta {
//     fn from(cell: MxCell) -> Self {
//         if let Ok(meta) = cell.get_widget_meta() {
//             return meta;
//         }
//         Default::default()
//     }
// }

// impl From<Element> for WidgetMeta {
//     fn from(e: Element) -> Self {
//         if let Ok(meta) = from_str::<WidgetMeta>(e.outer_html().as_str()) {
//             return meta;
//         }  
//         log::error!("can't create widget meta form: {}", e.outer_html());
//         Default::default()
//     }
// }


// ==========================================================
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn xml_widget_deser_works() {
        let xml = r#"<widget uuid="aaaaaaaaaa" name="test" group="group"/>"#;

        let widget = from_str::<Widget>(xml);    
        match widget {
            Ok(item) => {
                assert_eq!(item.uuid, "aaaaaaaaaa".to_owned());
                assert_eq!(item.name, "test".to_owned());
            },
            Err(err) => panic!("err: {}", err),
        }
    }    


}