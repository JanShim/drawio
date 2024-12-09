
use implicit_clone::unsync::IString;
use yew::{
    classes, function_component, html, Callback, Html, MouseEvent, Properties
};

use crate::model::widget::WidgetListItem;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub item: WidgetListItem,
    pub select: Callback<IString>,
    pub selected: IString,
}

#[function_component]
pub fn WidgetListItemComponent(Props {item, select, selected }: &Props) -> Html {
    let WidgetListItem {uuid, name, group } = item;

    let on_select = {
            let pk = uuid.clone();
            let select = select.clone();
            Callback::from(move |_: MouseEvent| {
                select.emit(pk.clone());
            })
        };

    html! {
        <div onclick={on_select} class={classes!( "selectable", (selected == uuid).then_some("selected") )}>
            {format!("{} {} {}", uuid, group, name)}
        </div>
    }
}