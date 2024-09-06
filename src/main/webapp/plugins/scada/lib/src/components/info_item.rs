
use yew::{
    function_component, html, Html, Properties
};

use crate::model::scada_diagram::meta::Diagram;


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

#[function_component(Component)]
pub fn scada_diagram_component(Props {item_type, name, uuid}: &Props) -> Html {
    html! {
        <div>
            { format!("uuid: {}, name: {}, type: {}", uuid, name, item_type) } 
        </div>
    }
}

