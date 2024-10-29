use implicit_clone::unsync::IString;
use serde::{Deserialize, Serialize};
use web_sys::FormData;
// use quick_xml::{de::from_str, se::to_string};

use super::NULL_UUID;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "widget")]
pub struct Widget {
    // #[serde(rename="@uuid")]    
    // pub uuid: IString,
    // #[serde(rename="@name")]    
    // pub name: IString,
    // #[serde(rename="@group")]    
    // pub group: IString,
    #[serde(rename="@object-type")]    
    pub object_type: IString,

}

impl Default for Widget {
    fn default() -> Self {
        Self { 
            // uuid: NULL_UUID.into(),
            // name: Default::default(),
            // group: Default::default(),
            object_type: Default::default(),
        }
    }
}

// impl From<FormData> for Widget {
//     fn from(data: FormData) -> Self {
//         Self { 
//             uuid: data.get("uuid").as_string().unwrap_or_default().into(), 
//             name: data.get("name").as_string().unwrap_or_default().into(), 
//             group: data.get("group").as_string().unwrap_or_default().into(), 
//         }
//     }
// }

#[derive(Debug, PartialEq, Clone)]
pub struct WidgetForm {
    pub uuid: IString,
    pub name: IString,
    pub group: IString,
}

impl WidgetForm {
    pub fn is_new_item(&self) -> bool {
        self.uuid == NULL_UUID
    }
}

impl Default for WidgetForm {
    fn default() -> Self {
        Self { 
            uuid: NULL_UUID.into(), 
            name: Default::default(),
            group: Default::default(),
        }
    }
}

impl From<FormData> for WidgetForm {
    fn from(data: FormData) -> Self {
        Self { 
            uuid: data.get("uuid").as_string().unwrap_or_default().into(), 
            name: data.get("name").as_string().unwrap_or_default().into(), 
            group: data.get("group").as_string().unwrap_or_default().into(), 
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
    use quick_xml::{de::from_str, se::to_string};

    use super::*;
    
    #[test]
    fn xml_deser_works() {
        let item = Widget::default();

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<Widget>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }    


}