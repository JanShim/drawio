use yew::{function_component, html, virtual_dom::VNode, Html};
use yewdux::{use_selector, use_store};

use crate::store::cell;



#[function_component(MultystateComponent)]
pub fn component() -> Html {
    let (state, dispatch) = use_store::<cell::State>();

    let multystate = use_selector(|state: &cell::State| 
            state.meta.clone()
            .map(|meta| meta.multystate)?
        );

    let multy: VNode = {
            let multy = multystate.clone();
            match *multy {
                Some(a) => html!{"here"},
                _ => html!{ "no multystate" }
            }
        };


    html! {
        <div>
            {"here multystate"}
        </div>
    }
    
}