
use yew::{
    function_component, html, AttrValue, Html, Properties
};

use super::meta::Diagram;
// use yew_autoprops::autoprops;

// use super::meta::ADiagram;

#[derive(PartialEq, Properties)]
pub struct Props {
    // pub meta: Diagram,
    pub item_type: String, 
    pub name: String, 
    pub uuid: String    
}

impl From<Diagram> for Props {
    fn from(Diagram {item_type, name, uuid}: Diagram) -> Self {
        Self {
            item_type,
            name,
            uuid,
        }
    }
}

// #[autoprops(InfoComponentProps)]
#[function_component(InfoComponent)]
pub fn scada_diagram_component(Props {item_type, name, uuid}: &Props) -> Html {
    //item_type: &AttrValue, name: &AttrValue, uuid: &AttrValue
    // let Diagram {item_type, name, uuid} = meta;
    html! {
        <div>
            { format!("uuid: {}, name: {}, type: {}", uuid, name, item_type) } 
        </div>
    }
}

