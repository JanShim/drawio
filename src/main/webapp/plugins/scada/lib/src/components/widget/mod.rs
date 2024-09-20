use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub edit_mode: bool,
}

#[function_component(WidgetComponent)]
pub fn component(Props { edit_mode }: &Props) -> Html {


    html! {
        <>
        // <pre>{ format!("{:?}", *multy_state) }</pre>
        <hr/>
        // <div class="flex-box delim-label">{"Состояния"}
        //     if *edit_mode {
        //         <button onclick={on_state_add}>{"+"}</button>
        //     } 
        // </div>
        // { states_view }
        </>
    }
}    