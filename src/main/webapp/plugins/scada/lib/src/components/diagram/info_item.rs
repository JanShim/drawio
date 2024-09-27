
use yew::{
    function_component, html, Html, Properties
};

use crate::components::widget;
use crate::model::common::{DiagramMeta, GraphModel};
use crate::utils::NULL_UUID;


#[derive(PartialEq, Properties)]
pub struct Props {
    // pub uuid: String,
    // pub name: String, 
    model: GraphModel,
}

impl From<DiagramMeta> for Props {
    fn from(DiagramMeta { label:_, model }: DiagramMeta) -> Self {
        match model {
            GraphModel::Diagram(diagram) => Self {
                model: GraphModel::Diagram(diagram),
            },
            GraphModel::Widget(widget) => Self {
                model: GraphModel::Widget(widget),
            },
        }
    }
}

#[function_component(Component)]
pub fn scada_diagram_component(Props { model }: &Props) -> Html {
    match model {
        GraphModel::Diagram(d) => html! {
            <div>
                { format!("diagram: uuid: {}, name: {}", d.uuid, d.name) } 
            </div>
        },
        GraphModel::Widget(w) =>     html! {
            <div>
                { format!("widget: uuid: {}, name: {}", w.uuid, w.name) } 
            </div>
        },
    }
        
    


}

