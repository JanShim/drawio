use implicit_clone::ImplicitClone;
use serde::{Deserialize, Serialize};
use quick_xml::{de::from_str, se::to_string};
use web_sys::Element;

use super::{diagram::meta::Diagram, mx_cell::MxCell, widget::meta::Widget};


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ImplicitClone)]
pub enum GraphModel {
    #[serde(rename="diagram")]    
    Diagram(Diagram),
    #[serde(rename="widget")]    
    Widget(Widget),
}

impl GraphModel {
    pub fn get_uuid(&self) -> String {
        match self {
            GraphModel::Diagram(diagram) => diagram.uuid.clone(),
            GraphModel::Widget(widget) => widget.uuid.clone(),
        }
    }
}

impl Default for GraphModel {
    fn default() -> Self {
        GraphModel::Diagram(Default::default())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
#[serde(rename = "object")]
pub struct DiagramMeta {
    #[serde(rename="@label")]    
    pub label: String,
    #[serde(rename="$value")]    
    pub model: GraphModel, 
}

impl DiagramMeta {
    pub fn get_uuid(&self) -> String {
        self.model.get_uuid()
    }
}


impl From<MxCell> for DiagramMeta {
    fn from(cell: MxCell) -> Self {
        if let Ok(meta) = cell.get_diagram_meta() {
            return meta;
        }
        Default::default()
    }
}

impl From<Element> for DiagramMeta {
    fn from(e: Element) -> Self {
        if let Ok(meta) = from_str::<DiagramMeta>(e.outer_html().as_str()) {
            return meta;
        }  
        log::error!("can't create diagram meta form: {}", e.outer_html());
        Default::default()
    }
}


// ==========================================================
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn xml_diagram_meta_deser_works() {
        let xml = r#"<object label="" id="0">
      <diagram uuid="aaaaaaaaaa" name="test"/>
    </object>"#;

        let diagram = from_str::<DiagramMeta>(xml);    
        match diagram {
            Ok(item) => {
                println!("{item:#?}");
                assert_eq!(item.get_uuid(), "aaaaaaaaaa".to_owned());

            },
            Err(err) => panic!("err: {}", err),
        }
    }

    #[test]
    fn xml_diagram_meta_ser_works() {
        let item = DiagramMeta {
            label: "".to_owned(),
            model: GraphModel::Diagram(Default::default()),

        };

        let str = to_string(&item).unwrap();
        println!("{str}");        

        let diagram = from_str::<DiagramMeta>(&str).unwrap();    

        assert_eq!(item, diagram);
    }
 

    #[test]
    fn xml_widget_meta_deser_works() {
        let xml = r#"<object label="" id="0">
      <widget uuid="aaaaaaaaaa" name="test" group="задвижки"/>
    </object>"#;

        let widget = from_str::<DiagramMeta>(xml);    
        match widget {
            Ok(item) => {
                println!("{item:#?}");
                assert_eq!(item.model.get_uuid(), "aaaaaaaaaa");
                // assert_eq!(item.widget.group, "задвижки");

            },
            Err(err) => panic!("err: {}", err),
        }
    }

    #[test]
    fn xml_widget_meta_ser_works() {
        let item = DiagramMeta {
            label: "".to_owned(),
            model: GraphModel::Widget( Widget {
                uuid: "aaaaaaaaaa".to_owned(),
                name: "test".to_owned(),
                ..Default::default()
            })
        };

        let str = to_string(&item).unwrap();
        println!("{str}");        

        let widget = from_str::<DiagramMeta>(&str).unwrap();    

        assert_eq!(item, widget);
    }

}