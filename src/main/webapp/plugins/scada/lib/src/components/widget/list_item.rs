
use yew::{
    function_component, html, Callback, Html, MouseEvent, Properties
};

use crate::model::widget::WidgetListItem;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub item: WidgetListItem,
    pub load: Callback<String>,
}

#[function_component(WidgetListItemComponent)]
pub fn component(Props {item, load}: &Props) -> Html {
    let WidgetListItem {uuid,name, group } = item;

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
            {format!("{} {} {}", uuid, group, name)} 
            // <button onclick={on_load}>{ "load" }</button>
        </div>
    }
}