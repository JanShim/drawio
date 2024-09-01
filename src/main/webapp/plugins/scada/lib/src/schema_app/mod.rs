
use std::rc::Rc;

use editor_ui::EditorUi;
use mx_editor::MxEditor;
use mx_graph::MxGraph;
use mx_utils::MxUtils;
use wasm_bindgen::prelude::*;
use yew::{
    prelude::*,
    // services::fetch::Request
};
use web_sys::HtmlDivElement;
// use wasm_bindgen_futures::spawn_local;
use yew_hooks::{use_async, use_async_with_options, UseAsyncOptions};

use crate::{
    //
    errors::FetchError, model::scada_diagram::{ScadaDiagramComponent, ScadaDiagramDto, ScadaDiagramListDto}, utils::{fetch, fetch_string, post} 
};

mod mx_utils;
mod mx_graph;
mod mx_graph_model;
mod mx_editor;
mod editor_ui;

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
    pub mx_utils: Rc<MxUtils>,
    pub mx_editor: Rc<MxEditor>,
}

#[function_component(App)]
pub fn app(props: &Props) -> Html {
    let url = props.api_url.clone();
    // let diagrams = use_list(Vec::<ScadaDiagramListDto>::new()); 

    // let state = use_async(
    //     async move { fetch::<Vec::<ScadaDiagramListDto>>(format!("{url}/diagram/all")).await },
    // );

    // let eitor = use_memo(props.mx_editor, |p| {
    //    p
    // });

    // let model_string = use_memo(props, |p| {
    //     if let Ok(node) = p.editor.get_graph_xml() {
    //         if let Ok(Some(v)) = p.mx_utils.get_pretty_xml(node) {
    //             return  v;
    //         }
    //     }
    //     "model error!".to_owned()
    // });

    let id = use_state(|| "00000000-0000-0000-0000-000000000000".to_owned());

    let diarrams_lis = use_async_with_options(
        async move { fetch::<Vec::<ScadaDiagramListDto>>(format!("{url}/diagram/all")).await },
        UseAsyncOptions::enable_auto(),
    );

    let model = use_state(|| "Not loaded".to_owned());
    let get_model = {
        let model = model.clone();
        let editor = props.mx_editor.clone();
        let utils = props.mx_utils.clone();

        Callback::from(move |_: MouseEvent| {
            if let Ok(node) = editor.get_graph_xml() {
                if let Ok(Some(v)) = utils.get_pretty_xml(node) {
                    model.set(v);
                    return;
                }
            }            

            model.set("model error!".to_owned());
        })
    };

    // load model from db
    let url = props.api_url.clone();
    let model_id = id.clone();
    let model_load = use_async(
        async move {
            let id = *model_id;
            fetch_string(format!("{url}/diagram/{id}/model")).await 
        }
    );
    // let on_load_model = {
    //     let handle = model_load.clone();
    //     Callback::from(move |_: MouseEvent| { handle.run(); })
    // };
    let on_load_model =  {
        let model_id = id.clone();
        let handle = model_load.clone();
        Callback::from(move |pk: String| { 
            let pk = pk.clone();
            model_id.set(pk.as_str());
            handle.run(); 
        })
    };

    // insert model to db
    let url = props.api_url.clone();
    let model = model.clone();
    let editor = props.mx_editor.clone();
    let utils = props.mx_utils.clone();    
    let model_create = use_async(
        async move {
            if let Ok(node) = editor.get_graph_xml() {
                if let Ok(Some(curr_model)) = utils.get_pretty_xml(node) {
                    let item = ScadaDiagramDto::new("insert proba".to_owned(), curr_model.clone());
                    return post(format!("{url}/diagram"), item).await;
                }
            }

            // let item = ScadaDiagramDto::new("insert proba".to_owned(), "<ssss/>".to_owned());
            // log::debug!("send post");
            // post(format!("{url}/diagram"), item).await

            // log::error!("can't insert model");
            Err(FetchError::InsertModelError("can't insert model".to_owned()))
        }
    );
    let on_create_model = {
        let handle = model_create.clone();
        Callback::from(move |_: MouseEvent| {
            handle.run();
        })        
     };

    // let state: yew_hooks::UseAsyncHandle<Vec<&str>, reqwasm::Error> = use_async(async move {
    //     let end_point = format!("{url}/diagram/all");

    //     // log::debug!("{}", &end_point);

    //     let fetched = Request::get(end_point.as_str()).send()
    //         .await
    //         .map(|o| {

    //             vec!["aaaa"]
    //         })
    //         .map_err(|err| {
    //             anyhow::Error::new(err)
    //         });
    //     // match fetched {
    //     //     Ok(response) => {
    //     //         let json = response.json::<Vec<ScadaDiagramListDto>>().await;
    //     //         match json {
    //     //             // Ok(f) => diagrams_clone.set(f),
    //     //             Ok(data) => data,
    //     //             Err(e) => {log::error!("{}", e); vec![]},
    //     //         }
    //     //     }
    //     //     Err(e) => {log::error!("{}", e); vec![]},
    //     // }   
    //     fetched         
    // });

    // let diagrams_clone = diagrams.clone();

    // spawn_local();

    html! {
        <>
            <p>{&props.val}</p>
            <p>{&props.api_url}</p>
            // <pre>{ for diagrams.current().iter()
            //     .map(|o| html!( <ScadaDiagramComponent item={o.clone()}/> )) 
            // }</pre>
            <pre>{
                if diarrams_lis.loading {
                    html! { "Loading, wait a sec..." }
                } else  {
                    diarrams_lis.data.as_ref().map_or_else(
                        || html! {},        // default
                        |repo| html! { 
                            for repo.iter().map(|o| 
                                html!{ <ScadaDiagramComponent item={o.clone()} load={on_load_model}/> }
                            )
                    })      
                }    
            }
            </pre>            
            <p>{
                diarrams_lis.error.as_ref().map_or_else(|| html! {}, |error| match error {
                    FetchError::SerdeError(err) => html! { err },
                    FetchError::RequestError(err) => html! { err },
                    FetchError::InsertModelError(err) => html!{ err },
                })
            }</p>            
            <div>
                <button onclick={get_model}>{ "Get" }</button>
                <button onclick={on_create_model}>{ "insert" }</button>
                // <button onclick={on_load_model} disabled={model_load.loading}>{ "Load" }</button>
                <pre>{ model.as_str() }</pre>
                <div>
                {
                    if model_load.loading {
                        html! { "Loading" }
                    } else {
                        html! {}
                    }
                }
                {
                    if let Some(data) = &model_load.data {
                        html! { data }
                    } else {
                        html! {}
                    }
                }
                {
                    if let Some(error) = &model_load.error {
                        html! { error }
                    } else {
                        html! {}
                    }
                }                
                </div>
                <div>
                {
                    if model_create.loading {
                        html! { "Creating..." }
                    } else {
                        html! {}
                    }
                }
                {
                    if let Some(data) = &model_create.data {
                        // model.set(serde_json::to_string(&data).unwrap());
                        html! { 
                            // model.as_str()
                            serde_json::to_string(&data).unwrap()
                            // data
                        }
                    } else {
                        html! {}
                    }
                }
                {
                    if let Some(error) = &model_create.error {
                        html! { error }
                    } else {
                        html! {}
                    }
                }                
                </div>

            </div>

        </>
    }    
}


#[wasm_bindgen(js_name=renderSchema)]
pub fn render_schema(mx_utils: MxUtils, mx_editor: MxEditor, div: HtmlDivElement, options: SchemaOptions) {
    let props  = Props {
        val: "SCHEMA".to_owned(),
        api_url: options.api_url.unwrap_or("undefiend".to_owned()),
        mx_utils: Rc::new(mx_utils),
        mx_editor: Rc::new(mx_editor),
    };
    yew::Renderer::<App>::with_root_and_props(div.into(), props).render();
    log::info!("schema loaded");
}




