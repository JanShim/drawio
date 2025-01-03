
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
    let WidgetListItem {uuid, name, name_ru, group } = item;

    let on_select = {
            let pk = uuid.clone();
            let select = select.clone();
            Callback::from(move |_: MouseEvent| {
                select.emit(pk.clone());
            })
        };

    html! {
        <tr onclick={on_select} class={classes!( "selectable", (selected == uuid).then_some("selected") )}>
            <td>{ name }</td>
            <td>{ name_ru.clone().unwrap_or_default() }</td>
            <td>{ group }</td>
        </tr>
    }
}