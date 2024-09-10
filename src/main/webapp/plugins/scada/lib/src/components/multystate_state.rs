use yew::{function_component, html, Html, Properties};
use yewdux::{use_selector, use_store};

use crate::store::cell;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub index: usize,
}

#[function_component(MultystateStateComponent)]
pub fn component(Props { index }: &Props) -> Html {
    let (state, _) = use_store::<cell::State>();
    let state = match state.get_multystate_state(*index).ok() {
        Some(state) => {
            html! {<div>{ format!("{state:#?}") }</div>}
        },
        _ => html! {<div>{"state not found"}</div>}
    }; 

    html!{
        { state }
    }
    
}