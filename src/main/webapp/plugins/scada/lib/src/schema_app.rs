
use wasm_bindgen::prelude::*;
use web_sys::HtmlDivElement;
use yew::{prelude::*, props};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub val: String,
}


#[function_component(App)]
pub fn app(props: &Props) -> Html {

    // let counter = use_state(|| 0);
    // let up = {
    //     let counter = counter.clone();
    //     Callback::from(move |_: MouseEvent| {
    //         counter.set(*counter + 1);
    //     })
    // };


    html! {
        <div>
            <p>{&props.val}</p>
        </div>
    }    
}


#[wasm_bindgen(js_name=renderSchema)]
pub fn render_schema(div: HtmlDivElement) {
    let props  = Props {val: "SCHEMA".to_owned()};
    yew::Renderer::<App>::with_root_and_props(div.into(), props).render();
    log::info!("schema loaded");
}
