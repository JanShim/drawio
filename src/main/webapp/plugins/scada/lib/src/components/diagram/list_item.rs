
use yew::{
    function_component, html, Callback, Html, MouseEvent, Properties
};

use crate::model::diagram::DiagramListItem;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub item: DiagramListItem,
    pub load: Callback<String>,
}

#[function_component(DiagramListItemComponent)]
pub fn component(Props {item, load}: &Props) -> Html {
    let DiagramListItem {uuid, name} = item;

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
            // <button onclick={on_load}>{ "load" }</button>
        </div>
    }
}