
use serde::{Deserialize, Serialize};
use yew::{
    function_component, html, Callback, Html, MouseEvent, Properties
};


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
pub struct ListItem {
    pub uuid: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct DiagramDto {
    pub uuid: String,
    pub name: String,
    pub model: String,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub item: ListItem,
    pub load: Callback<String>,
}

#[function_component(ListItemComponent)]
pub fn scada_diagram_component(props: &Props) -> Html {
    let Props {item, load} = props;
    let ListItem {uuid, name} = item;

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