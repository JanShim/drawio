
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use reqwasm::http::Request;
use web_sys::HtmlDivElement;
use wasm_bindgen_futures::spawn_local;
use yew_hooks::use_list;

use crate::scada_object::{ScadaObject, ScadaObjectComponent};

#[wasm_bindgen]
pub struct SchemaOptions {
    #[wasm_bindgen(skip)]
    pub api_url: Option<String>,
}

#[wasm_bindgen]
impl SchemaOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(api_url: Option<String>) -> Self {
        Self { 
            api_url,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub val: String,
    pub api_url: String,
}

#[function_component(App)]
pub fn app(props: &Props) -> Html {
    let url = props.api_url.clone();
    let objects = use_list(Vec::<ScadaObject>::new()); 

    let object_clone = objects.clone();
    spawn_local(async move {
        let end_point = format!("{url}/object-type/14/objects");
        let fetched = Request::get(end_point.as_str()).send().await;
        match fetched {
            Ok(response) => {
                let json = response.json::<Vec<ScadaObject>>().await;
                match json {
                    Ok(f) => object_clone.set(f),
                    Err(e) => log::error!("{}", e),
                }
            }
            Err(e) => log::error!("{}", e),
        }            
    });

    html! {
        <>
            <p>{&props.val}</p>
            <p>{&props.api_url}</p>
            <pre>{ for objects.current().iter()
                .map(|o| html!( <ScadaObjectComponent item={o.clone()}/> )) 
            }</pre>
        </>
    }    
}


#[wasm_bindgen(js_name=renderSchema)]
pub fn render_schema(div: HtmlDivElement, options: SchemaOptions) {
    let props  = Props {
        val: "SCHEMA".to_owned(),
        api_url: options.api_url.unwrap_or("undefiend".to_owned()),
    };
    yew::Renderer::<App>::with_root_and_props(div.into(), props).render();
    log::info!("schema loaded");
}
