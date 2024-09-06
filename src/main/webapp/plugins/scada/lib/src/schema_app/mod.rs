use wasm_bindgen::prelude::*;
use yew::prelude::*;
use std::rc::Rc;

use web_sys::HtmlDivElement;
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use js_functions::{get_cell0, load_scada_model};

use crate::{
    components::{
        info_item::Component as InfoComponent, list_item::Component as ListItemComponents
    }, 
    errors::FetchError, 
    model::{mx_editor::MxEditor, mx_utils::MxUtils, scada_diagram::{
        meta::Meta as DiagramMeta, 
        ListItem, 
        ScadaDiagramDto, NULL_UUID
    }}, 
    utils::{fetch, fetch_string, post} 
};

pub mod js_functions;

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
pub fn app(Props {val, api_url, mx_utils, mx_editor}: &Props) -> Html {
    let editor = mx_editor.clone();
    let meta = use_state(|| {
        let cell_meta: DiagramMeta = get_cell0(&editor).into();
        log::debug!("loaded meta {:#?}", cell_meta);
        cell_meta
    });

    let url = api_url.clone();
    let diagram_list = use_async_with_options(
        async move { fetch::<Vec::<ListItem>>(format!("{url}/diagram/all")).await },
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

    // let url = props.api_url.clone();
    // let id_clone = id.clone();
    // let model_load_handle = use_async(
    //     async move {
    //         fetch_string(format!("{url}/diagram/{}/model", *id_clone)).await
    //             .and_then(|text| {
    //                 // let id = id_clone.clone();
    //                 let meta = load_scada_model(&*editor, text.as_str());
    //                 log::debug!("meta: {:#?}", meta);
    //                 match meta {
    //                     str if str.is_string() => {
    //                         let xml_str = str.as_string().expect("must be string");
    //                         let meta = serde_xml_rs::from_str::<scada_diagram::meta::Meta>(&xml_str).unwrap();
    //                         log::debug!("meta: {:#?}", meta);
    //                     }, 
    //                     _ => {
    //                     },
    //                 }
    //                 // log::debug!("{}", meta);
    //                 Ok(())
    //             }) 
    //     }
    // );
    // let model_load_handle_clone = model_load_handle.clone();
    // use_effect_with(id.clone(), move |id| {
    //     if !(*id).as_str().eq(NULL_UUID) {
    //         log::debug!("clicked {}", (*id).to_string());
    //         // model_load_handle_clone.run();
    //     }
    // });

    // ---------------
    // let on_load_model =  {
    //     let id = id.clone();
    //     Callback::from(move |pk: String|  {
    //         let editor = editor.clone();
    //         let url = url.clone();
    //         let id = id.clone();
    //         wasm_bindgen_futures::spawn_local(async move {
    //             fetch_string(format!("{url}/diagram/{pk}/model")).await  
    //             .map(|model| {
    //                 let meta = match load_scada_model(&editor, model.as_str()) {
    //                     str if str.is_string() => {
    //                         let xml_str = str.as_string().expect("must be string");
    //                         serde_xml_rs::from_str::<scada_diagram::meta::Meta>(&xml_str)
    //                             .map_err(|err| {log::debug!("can't deserialize schema meta"); err})
    //                             .unwrap()
    //                     }, 
    //                     _ => {
    //                         scada_diagram::meta::Meta {
    //                             label: "".to_owned(),
    //                             diagram: scada_diagram::meta::Diagram { 
    //                                 item_type: "schema".to_owned(), 
    //                                 uuid: NULL_UUID.to_owned(), 
    //                             }
    //                         }
    //                     },
    //                 };                    
    //                 log::debug!("meta: {:#?}", meta);
    //                 let uuid = meta.diagram.uuid;
    //                 id.set(uuid);
    //             }).unwrap();
    //         });
    //     })
    // };


    // ---------------
    // load model from db
    let on_load_model =  {
        let editor = mx_editor.clone();
        let url = api_url.clone();
        Callback::from(move |pk: String|  {
            let editor = editor.clone();
            let url = url.clone();
            wasm_bindgen_futures::spawn_local(async move {
                fetch_string(format!("{url}/diagram/{pk}/model")).await  
                .map(|model| {
                    load_scada_model(&editor, model.as_str());
                }).unwrap();
            });
        })
    };


    // ---------------
    // insert model to db
    let on_create_model =  {
        let editor = mx_editor.clone();
        let utils = mx_utils.clone();
        let url = api_url.clone();
        Callback::from(move |_: MouseEvent|  {
            let editor = editor.clone();
            let utils = utils.clone();
            let url = url.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(node) = editor.get_graph_xml() {
                    if let Ok(Some(model_str)) = utils.get_xml(node) {
                        let item = ScadaDiagramDto::new("insert proba".to_owned(), model_str);
                        post(format!("{url}/diagram"), item).await
                            .and_then(|o| Ok(o.uuid))
                            .map(|pk| {
                                wasm_bindgen_futures::spawn_local(async move {
                                    fetch_string(format!("{url}/diagram/{pk}/model")).await
                                        .map(|model| {
                                            load_scada_model(&editor, model.as_str());
                                        }).unwrap();
                                })
                            })
                            .unwrap();
                    } 
                } 
            });
        })
    };

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
            <p>{val}</p>
            <p>{api_url}</p>
             <pre>{
                if diagram_list.loading {
                    html! { "Loading, wait a sec..." }
                } else  {
                    diagram_list.data.as_ref().map_or_else(
                        || html! {},        // default
                        |repo| html! { 
                            for repo.iter().map(|item| 
                                html!{ <ListItemComponents item={item.clone()} load={on_load_model.clone()}/> }
                            )
                    })      
                }    
            }
            </pre>            
            <p>{
                diagram_list.error.as_ref().map_or_else(|| html! {}, |error| match error {
                    FetchError::SerdeError(err) => html! { err },
                    FetchError::RequestError(err) => html! { err },
                    FetchError::InsertModelError(err) => html!{ err },
                    FetchError::ParseXmlError(err) => html!{ err },
                })
            }</p>
            <div>
                <button onclick={on_create_model} disabled={(*meta).clone().diagram.uuid.ne(NULL_UUID)}>{ "insert" }</button >
            </div>
            <InfoComponent ..(*meta).clone().diagram.into() /> 
            <div>
                // <button onclick={get_model}>{ "Get" }</button>
                // <button onclick={on_load_model} disabled={model_load.loading}>{ "Load" }</button>
                // <pre> { &*id  } </pre>
                // <div>
                // {
                //     if model_load_handle.loading {
                //         html! { "Loading" }
                //     } else {
                //         html! {}
                //     }
                // }
                // // {
                // //     if let Some(data) = &model_load_handle.data {
                // //         html! { data }
                // //     } else {
                // //         html! {}
                // //     }
                // // }
                // {
                //     if let Some(error) = &model_load_handle.error {
                //         html! { error }
                //     } else {
                //         html! {}
                //     }
                // }                
                // </div>
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




