use yew::{function_component, html, Html};
use yewdux::use_store;

use crate::store::cell;



#[function_component(CellDetailsComponent)]
pub fn component() -> Html {
    let (state, dispatch) = use_store::<cell::State>();

    // let up = dispatch.reduce_mut_callback(|state| {
    //     // let aaa = state.cell

    //         // if let Some(cell) = state.cell {
    //         //     cell.append_meta_element(|root| {
    //         //         let doc = root.owner_document().unwrap();

    //         //         let aaa = doc.create_element("aaaa").unwrap();
    //         //         &aaa
    //         //     })
    //         // }
    //     });    


    match &state.cell {
        Some(cell) => html! {
            <div>
            <pre>{ cell.get_meta_xml() }</pre>
            </div>
        },
        _ => html! {<div></div>},
    }
}