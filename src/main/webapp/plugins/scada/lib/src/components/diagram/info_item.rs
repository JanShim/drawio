
use yew::prelude::*;

use crate::model::{common::{DiagramMeta, GraphModel}, diagram::meta::Diagram};


#[derive(PartialEq, Properties)]
pub struct Props {
    pub diagram: Diagram,
}

#[function_component(DiagramInfoComponent)]
pub fn scada_diagram_component(Props { diagram }: &Props) -> Html {
    let edit_mode = use_state(|| false);
    

    let edit_mode_toggle = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |_: MouseEvent| { edit_mode.set(true); })
    };    

    let cell_details_apply: Callback<MouseEvent> = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |_: MouseEvent| {
            // log::debug!("NEW CELL META:: {:?}", meta);
            edit_mode.set(false);
        })
    }; 

    let header = html!{
        <div class="flex-box-2 delim-label" >
        if *edit_mode {
            <button onclick={cell_details_apply}><img src="images/checkmark.gif" width="16" height="16"/></button>
        } else {
            <button onclick={edit_mode_toggle}><img src="images/edit16.png"/></button>
        }
        </div>           
    };

    html! {
        <>
        {header}
        <div>
            { format!("diagram: uuid: {}, name: {}", diagram.uuid, diagram.name) } 
        </div>
        </>
    }
}

