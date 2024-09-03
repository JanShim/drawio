use serde::{Deserialize, Serialize};
use yew::{
    function_component, html, use_state, Callback, Html, MouseEvent, Properties
};

pub mod meta;


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
pub struct ScadaDiagramListDto {
    pub uuid: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ScadaDiagramDto {
    pub uuid: String,
    pub name: String,
    pub model: String,
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
pub struct Props {
    pub item: ScadaDiagramListDto,
    pub load: Callback<String>,
}

#[function_component(DiagramListItem)]
pub fn scada_diagram_component(props: &Props) -> Html {
    let Props {item, load} = props;
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


