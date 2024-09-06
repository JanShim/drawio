use yew::{function_component, html, Html};
use yewdux::use_store;

use crate::store::cell;



#[function_component(CellDetailsComponent)]
pub fn component() -> Html {
    let (state, _dispatch) = use_store::<cell::State>();



    let cell = (*state);
    html! {
        <div>
            { format!("label {:#?}", cell) } 
        </div>
    }
}