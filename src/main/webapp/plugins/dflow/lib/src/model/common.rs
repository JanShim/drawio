use implicit_clone::ImplicitClone;
use serde::{Deserialize, Serialize};
use quick_xml::de::from_str;
use web_sys::Element;
use common_model::diagram::{DiagramXml, WidgetXml};

use super::{
    diagram::form::DiagramForm,
    mx_cell::MxCell,
    widget::form::WidgetForm
};


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ImplicitClone)]
pub enum GraphModel {
    #[serde(rename="diagram")]
    Diagram(DiagramXml),
    #[serde(rename="widget")]
    Widget(WidgetXml),
}

impl GraphModel {
    // pub fn get_uuid(&self) -> String {
    //     match self {
    //         GraphModel::Diagram(diagram) => diagram.uuid.to_string(),
    //         GraphModel::Widget(widget) => widget.uuid.to_string(),
    //     }
    // }
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
    pub fn get_widget_default() -> Self {
        Self {
            label: Default::default(),
            model: GraphModel::Widget(Default::default()),
        }
    }

    pub fn get_diagram_default() -> Self {
        Self {
            label: Default::default(),
            model: GraphModel::Diagram(Default::default()),
        }
    }
}

// impl From<MxCell> for DiagramMeta {
//     fn from(cell: MxCell) -> Self {
//         if let Ok(meta) = cell.get_diagram_meta() {
//             return meta;
//         }
//         Default::default()
//     }
// }

// impl From<Element> for DiagramMeta {
//     fn from(e: Element) -> Self {
//         if let Ok(meta) = from_str::<DiagramMeta>(e.outer_html().as_str()) {
//             return meta;
//         }
//         log::error!("can't create diagram meta form: {}", e.outer_html());
//         Default::default()
//     }
// }

#[derive(PartialEq, Debug, Clone, ImplicitClone)]
pub enum ModelForm {
    Diagram(DiagramForm),
    Widget(WidgetForm),
}

impl Default for ModelForm {
    fn default() -> Self {
        ModelForm::Diagram(Default::default())
    }
}

// ==========================================================
#[cfg(test)]
mod tests {
    use common_model::{data_source::DataSourceXml, diagram::WidgetPropertyXml};
    use quick_xml::se::to_string;

    use super::*;

    #[test]
    fn xml_diagram_meta_ser_works() {
        let item = DiagramMeta::get_diagram_default();

        let str = to_string(&item).unwrap();
        println!("{str}");

        let diagram = from_str::<DiagramMeta>(&str).unwrap();

        assert_eq!(item, diagram);
    }


    #[test]
    fn xml_widget_meta_deser_works() {
        let item = DiagramMeta {
            label: "".to_owned(),
            model: GraphModel::Widget(WidgetXml {
                object_type: "ZDV".to_owned(),
                property: vec![WidgetPropertyXml {
                    name: "".to_owned(),
                    ds: DataSourceXml::default(),
                }]
            }),
        };

        let xml = quick_xml::se::to_string(&item).unwrap();
        println!("{xml}");

        // let xml = r#"<object label="" id="0"><widget object-type=""/></object>"#;

        let widget = quick_xml::de::from_str::<DiagramMeta>(&xml);
        match widget {
            Ok(item) => {
                println!("{item:#?}");
            },
            Err(err) => panic!("err: {}", err),
        }
    }

}