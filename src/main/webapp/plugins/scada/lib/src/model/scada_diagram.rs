use std::rc::Rc;

use serde::{Deserialize, Serialize};
use yew::{
    function_component, html, use_callback, use_state, Callback, Event, Html, MouseEvent, Properties
};


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
pub struct ScadaDiagramListDto {
    uuid: String,
    name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ScadaDiagramDto {
    uuid: String,
    name: String,
    model: String,
}

impl ScadaDiagramDto {
    pub fn new(name: String, model: String) -> Self {
        ScadaDiagramDto {
            uuid: "00000000-0000-0000-0000-000000000000".to_owned(),
            name,
            model,
        }
    }
}


#[derive(PartialEq, Properties)]
pub struct ScadaDiagramComponentProps {
    pub item: ScadaDiagramListDto,
    pub load: Callback<String>,
}

#[function_component(ScadaDiagramComponent)]
pub fn scada_diagram_component(props: &ScadaDiagramComponentProps) -> Html {
    let ScadaDiagramComponentProps {item, load} = props;
    let ScadaDiagramListDto {uuid, name} = item;

    let on_load = {
            let pk = uuid.clone();
            let load = load.clone();
            Callback::from(move |_: MouseEvent| {
                let pk = pk.clone();
                load.emit(pk);
            })
        };

    html! {
        <div>
            {format!("{} {}", uuid, name)} 
            <button onclick={on_load}>{ "load" }</button>
        </div>
    }
}