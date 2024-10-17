use implicit_clone::sync::IString;
use yew::{function_component, html, use_effect_with, use_memo, use_reducer, AttrValue, Html, Properties};

use crate::{
    components::multystate::state_rect::{self, StateSampleRect}, 
    model::cell_meta::multystate::state::{StateAction, StateMeta}, utils::{map_to_svg_style, mx_style_to_map, string_to_map} 
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

    let style_string = use_memo(my_state.style.clone(), |style| {
        let map = mx_style_to_map(style);       
        AttrValue::from(map_to_svg_style(map))
    });


    // --- view items
    let view_mode = html! {
        <table>
        <tr>
            <td width="200">{ my_state.name.as_str() }</td>
            <td>{"знач: "}</td>
            <td width="35">{ my_state.value.to_string() }</td>
            <td width="50"><StateSampleRect style={(*style_string).clone()}/></td>
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