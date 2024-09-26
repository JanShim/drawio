
use yew::{
    function_component, html, Html, Properties
};

use crate::model::scada_diagram::meta::Diagram;


#[derive(PartialEq, Properties)]
pub struct Props {
    pub uuid: String,
    pub name: String, 
}

impl From<Diagram> for Props {
    fn from(Diagram {name, uuid}: Diagram) -> Self {
        Self {
            name,
            uuid,
        }
    }
}

#[function_component(Component)]
pub fn scada_diagram_component(Props {name, uuid}: &Props) -> Html {
    html! {
        <div>
            { format!("uuid: {}, name: {}", uuid, name) } 
        </div>
    }
}

