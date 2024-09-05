use wasm_bindgen::prelude::*;
use yew::{prelude::*, virtual_dom::Attributes, html};
use std::rc::Rc;

use mx_editor::MxEditor;
use mx_utils::MxUtils;
use web_sys::HtmlDivElement;
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use js_functions::{get_cell0, load_scada_model};

use crate::{
    errors::FetchError, 
    model::scada_diagram::{
        self, info_item::InfoComponent, 
        list_item::{ListItem, ListItemComponent}, 
        meta::{AMeta as DiagramAMeta, Meta as DiagramMeta}
    }, utils::{fetch, fetch_string, post} 
};

pub mod mx_utils;
pub mod mx_graph;
pub mod mx_graph_model;
pub mod mx_editor;
pub mod mx_cell;
pub mod editor_ui;
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
    // let aaa = AttrValue::from(props.api_url.clone());
    // let utils = props.mx_utils.clone();    
    // let diagrams = use_list(Vec::<ScadaDiagramListDto>::new()); 

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
    let editor = mx_editor.clone();
    let meta = use_state(|| {
        let cell_meta: DiagramMeta = get_cell0(&editor).into();
        log::debug!("loaded meta {:#?}", cell_meta);
        cell_meta
    });

    // let id = use_state_eq(|| NULL_UUID.to_owned());
    // let editor = props.mx_editor.clone();
    // let meta = use_state_eq(|| None);

    let url = api_url.clone();
    let diagram_list = use_async_with_options(
        async move { fetch::<Vec::<ListItem>>(format!("{url}/diagram/all")).await },
        UseAsyncOptions::enable_auto(),
    );

    // let meta = use_state(||DiagramMeta::default());
    // let uuid = use_state(|| "value uuid".to_owned());
    // let aaa = use_state(|| 0);

   
    // let uuid = use_memo(meta.clone(), |m | {(*m).diagram.uuid.clone()});


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
    let on_load_model =  {
        let editor = mx_editor.clone();
        let url = api_url.clone();
        // let meta = meta.clone();
        // let uuid = uuid.clone();
        // let aaa = aaa.clone();
        Callback::from(move |pk: String|  {
            let editor = editor.clone();
            let url = url.clone();
            // let meta = meta.clone();
            // let uuid = uuid.clone();
            // let aaa = aaa.clone();
            // aaa.set(321);
            // log::debug!("aaaaaaa {}", *aaa);
            wasm_bindgen_futures::spawn_local(async move {
                fetch_string(format!("{url}/diagram/{pk}/model")).await  
                .map(|model| {
                    load_scada_model(&editor, model.as_str());
                    // let cell_meta: scada_diagram::meta::Meta = get_cell0(&editor).into();
                    // log::debug!("loaded meta {:#?}", cell_meta);
                    // let pk = cell_meta.diagram.uuid.clone();
                    // meta.set(cell_meta);
                    // log::debug!("----{pk}");
                    // uuid.set(pk);
                    // aaa.set(123);
                }).unwrap();
            });
        })
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

    // let uuid = uuid.clone();

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
                                html!{ <ListItemComponent item={item.clone()} load={on_load_model.clone()}/> }
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
            // <InfoComponent meta={(*meta).clone().diagram}/> 
            <InfoComponent ..(*meta).clone().diagram.into() /> 
            <div>
                // <button onclick={get_model}>{ "Get" }</button>
                // <button onclick={on_create_model}>{ "insert" }</button>
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




