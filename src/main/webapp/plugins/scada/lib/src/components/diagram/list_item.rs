
use implicit_clone::unsync::IString;
use yew::{
    classes, function_component, html, Callback, Html, MouseEvent, Properties
};

use crate::model::diagram::DiagramListItem;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub item: DiagramListItem,
    pub select: Callback<IString>,
    pub selected: IString,
}

#[function_component(DiagramListItemComponent)]
pub fn component(Props {item, select, selected }: &Props) -> Html {
    let DiagramListItem {uuid, name} = item;

    let on_select = {
            let pk = uuid.clone();
            let select = select.clone();
            Callback::from(move |_: MouseEvent| {
                select.emit(pk.clone());
            })
        };

    html! {
        <div onclick={on_select} class={classes!( "selectable", (selected == uuid).then_some("selected") )}>
            {format!("{} {}", uuid, name)} 
        </div>
    }
}