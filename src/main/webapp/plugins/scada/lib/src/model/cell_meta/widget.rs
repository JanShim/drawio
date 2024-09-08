use serde::{Deserialize, Serialize};

pub fn is_none_widget(tst: &Option<WidgetMeta>) -> bool {
    match tst {
        Some(_) => false,
        None => true,
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "widget")]
pub struct WidgetMeta {
    #[serde(rename="@uuid")]
    pub uuid: String,
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
    fn xml_widget_meta_serde_works() {
        let item = WidgetMeta {
            uuid: "some".to_owned(),
        };

        let str = to_string(&item).unwrap();
        println!("{str}");

        let meta = from_str::<WidgetMeta>(&str).unwrap();
        println!("{meta:#?}");

        assert_eq!(item, meta);
    }
   

}