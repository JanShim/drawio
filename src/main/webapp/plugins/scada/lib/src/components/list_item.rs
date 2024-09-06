
use yew::{
    function_component, html, Callback, Html, MouseEvent, Properties
};

use crate::model::scada_diagram::ListItem;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub item: ListItem,
    pub load: Callback<String>,
}

#[function_component(Component)]
pub fn scada_diagram_component(Props {item, load}: &Props) -> Html {
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