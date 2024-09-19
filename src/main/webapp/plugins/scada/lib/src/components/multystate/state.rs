use yew::{html, function_component, use_effect_with, use_reducer, Html, Properties};

use crate::{
    model::cell_meta::{
        // CellMeta,
        multystate::state::{StateAction, StateMeta}, 
    }, 
    // store::cell,
};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub value: StateMeta,
}

#[function_component(MultystateStateComponent)]
pub fn component(Props {
    value, 
}: &Props) -> Html {
    let my_state = use_reducer(|| value.clone());
    {
        let my_state = my_state.clone();
        use_effect_with(value.clone(), move |value| {
            my_state.dispatch(StateAction::Clone((*value).clone()));
        });
    }


    // --- view items
    let view_mode = html! {
        <table>
        <tr>
            <td width="200">{ my_state.name.as_str() }</td>
            <td>{"знач: "}</td>
            <td width="35">{ my_state.range.to_string() }</td>
            <td width="50">
                <svg viewBox="0 0 40 20" width="40" height="20" xmlns="http://www.w3.org/2000/svg"><rect x="0" y="0" width="100%" height="100%"></rect></svg>
            </td>
        </tr>
        </table>    
    };

    // item view
    html! {
        <table class="prop-table">
        // <td class="label" width="10">{my_state.pk.clone()}</td>
        <td>{ view_mode }</td>
        <td class="img"></td>
        </table>
    }
    
}