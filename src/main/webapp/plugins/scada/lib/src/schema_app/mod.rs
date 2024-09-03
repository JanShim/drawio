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
use web_sys::{js_sys::JsString, HtmlDivElement};
// use wasm_bindgen_futures::spawn_local;
use yew_hooks::{use_async, use_async_with_options, UseAsyncOptions};

use crate::{
    //
    errors::FetchError, 
    model::scada_diagram::{DiagramListItem, ScadaDiagramDto, ScadaDiagramListDto}, 
    utils::{fetch, fetch_string, post} 
};

mod mx_utils;
mod mx_graph;
mod mx_graph_model;
mod mx_editor;
mod editor_ui;

const NULL_UUID: &str = "00000000-0000-0000-0000-000000000000";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name=loadScadaModel)]
    pub fn load_scada_model(editor: &MxEditor, xmlStr: &str) -> JsValue;
}

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
    let editor = props.mx_editor.clone();
    // let utils = props.mx_utils.clone();    
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

    let id = use_state(|| NULL_UUID.to_owned());

    let diagram_list = use_async_with_options(
        async move { fetch::<Vec::<ScadaDiagramListDto>>(format!("{url}/diagram/all")).await },
        UseAsyncOptions::enable_auto(),
    );

    // let model = use_state(|| "Not loaded".to_owned());
    // let get_model = {
    //     let model = model.clone();
    //     let editor = props.mx_editor.clone();
    //     let utils = props.mx_utils.clone();

    //     Callback::from(move |_: MouseEvent| {
    //         if let Ok(node) = editor.get_graph_xml() {
    //             if let Ok(Some(v)) = utils.get_pretty_xml(node) {
    //                 model.set(v);
    //                 return;
    //             }
    //         }            

    //         model.set("model error!".to_owned());
    //     })
    // };

    // load model from db
    let url = props.api_url.clone();
    let id_clone = id.clone();
    let model_load_handle = use_async(
        async move {
            fetch_string(format!("{url}/diagram/{}/model", *id_clone)).await
                .and_then(|text| {
                    let meta = load_scada_model(&*editor, text.as_str());
                    match meta {
                        node if node.is_string() => {
                            // serde_xml_rs::from_str::<DiagramMeta>(node.)
                        }, 
                        _ => {

                        },
                    }
                    // log::debug!("{}", meta);
                    Ok(())
                }) 
        }
    );
    let model_load_handle_clone = model_load_handle.clone();
    use_effect_with(id.clone(), move |id| {
        if !(*id).as_str().eq(NULL_UUID) {
            log::debug!("clicked {}", (*id).to_string());
            model_load_handle_clone.run();
        }
    });
    let on_load_model =  {
        let id = id.clone();
        Callback::from(move |pk: String| id.set(pk))
    };

    // // insert model to db
    // let url = props.api_url.clone();
    // // let model = model.clone();
    // // let id_clone = id.clone();
    // let editor = props.mx_editor.clone();
    // let utils = props.mx_utils.clone();    
    // let model_create = use_async(
    //     async move {
    //         if let Ok(node) = editor.get_graph_xml() {
    //             if let Ok(Some(curr_model)) = utils.get_pretty_xml(node) {
    //                 let item = ScadaDiagramDto::new("insert proba".to_owned(), curr_model.clone());
    //                 return post(format!("{url}/diagram"), item).await
    //                     .and_then(|o| {
    //                         log::debug!("new {}", o.uuid);
    //                         // id_clone.set(o.uuid);
    //                         Ok(())
    //                     });
    //             }
    //         }

    //         Err(FetchError::InsertModelError("can't insert model".to_owned()))
    //     }
    // );
    // let on_create_model = {
    //     let handle = model_create.clone();
    //     Callback::from(move |_: MouseEvent| {
    //         handle.run();
    //     })        
    //  };


    html! {
        <>
            <p>{&props.val}</p>
            <p>{&props.api_url}</p>
            // <pre>{ for diagrams.current().iter()
            //     .map(|o| html!( <ScadaDiagramComponent item={o.clone()}/> )) 
            // }</pre>
            <pre>{
                if diagram_list.loading {
                    html! { "Loading, wait a sec..." }
                } else  {
                    diagram_list.data.as_ref().map_or_else(
                        || html! {},        // default
                        |repo| html! { 
                            for repo.iter().map(|item| 
                                html!{ <DiagramListItem item={item.clone()} load={on_load_model.clone()}/> }
                            )
                    })      
                }    
            }
            </pre>            
            <p >{
                diagram_list.error.as_ref().map_or_else(|| html! {}, |error| match error {
                    FetchError::SerdeError(err) => html! { err },
                    FetchError::RequestError(err) => html! { err },
                    FetchError::InsertModelError(err) => html!{ err },
                    FetchError::ParseXmlError(err) => html!{ err },
                })
            }</p>            
            <div>
                // <button onclick={get_model}>{ "Get" }</button>
                // <button onclick={on_create_model}>{ "insert" }</button>
                // <button onclick={on_load_model} disabled={model_load.loading}>{ "Load" }</button>
                <pre>{ id.as_str()  }
                </pre>
                <div>
                {
                    if model_load_handle.loading {
                        html! { "Loading" }
                    } else {
                        html! {}
                    }
                }
                // {
                //     if let Some(data) = &model_load_handle.data {
                //         html! { data }
                //     } else {
                //         html! {}
                //     }
                // }
                {
                    if let Some(error) = &model_load_handle.error {
                        html! { error }
                    } else {
                        html! {}
                    }
                }                
                </div>
                // <div>
                // {
                //     if model_create.loading {
                //         html! { "Creating..." }
                //     } else {
                //         html! {}
                //     }
                // }
                // // {
                // //     if let Some(data) = &model_create.data {
                // //         // model.set(serde_json::to_string(&data).unwrap());
                // //         html! { 
                // //             // model.as_str()
                // //             serde_json::to_string(&data).unwrap()
                // //             // data
                // //         }
                // //     } else {
                // //         html! {}
                // //     }
                // // }
                // {
                //     if let Some(error) = &model_create.error {
                //         html! { error }
                //     } else {
                //         html! {}
                //     }
                // }                
                // </div>

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




