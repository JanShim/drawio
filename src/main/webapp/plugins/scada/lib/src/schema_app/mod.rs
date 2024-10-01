use implicit_clone::unsync::IString;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yewdux::use_dispatch;
use std::rc::Rc;

use web_sys::HtmlDivElement;

use crate::{
    // errors::FetchError, 
    components::{get_global_css, InfoComponent}, model::{
        common::{DiagramMeta, GraphModel}, 
        diagram::ScadaDiagramDto, 
        mx_editor::MxEditor, 
        mx_utils::MxUtils, widget::WidgetDto
    }, store::diagram, utils::{fetch_string, get_cell0, load_scada_model, post, SchemaOptions} 
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub api_url: IString,
    pub mx_utils: Rc<MxUtils>,
    pub mx_editor: Rc<MxEditor>,
}

#[function_component(App)]
pub fn app(Props {api_url, mx_utils, mx_editor}: &Props) -> Html {
    let dispatch = use_dispatch::<diagram::State>();

    let url = api_url.clone();
    let utils = mx_utils.clone();
    let editor = mx_editor.clone();
    // This runs only once, on the first render of the component.
    use_effect_with(
        (), // empty deps
        move |_| {
            dispatch.set( diagram::State { 
                api_url: url.to_string(), 
                mx_utils: Some(utils),
                mx_editor: Some(editor),
            });
            || {}
        },
    );  

    let editor = mx_editor.clone();
    let meta = use_state(|| {
        let cell_meta: DiagramMeta = get_cell0(&editor).into();
        log::debug!("loaded meta {:#?}", cell_meta);
        cell_meta
    });

    // let url = api_url.clone();
    // let diagram_list = use_async_with_options(
    //     async move { fetch::<Vec::<DiagramListItem>>(format!("{url}/diagram/all")).await },
    //     UseAsyncOptions::enable_auto(),
    // );

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

    // // ---------------
    // // load model from db
    // let on_load_model =  {
    //     let editor = mx_editor.clone();
    //     let url = api_url.clone();
    //     Callback::from(move |pk: String|  {
    //         let editor = editor.clone();
    //         let url = url.clone();
    //         wasm_bindgen_futures::spawn_local(async move {
    //             fetch_string(format!("{url}/diagram/{pk}/model")).await  
    //             .map(|model| {
    //                 load_scada_model(&editor, model.as_str());
    //             }).unwrap();
    //         });
    //     })
    // };


    // // ---------------
    // // insert model to db
    // let on_create_model =  {
    //     let editor = mx_editor.clone();
    //     let utils = mx_utils.clone();
    //     let url = api_url.clone();
    //     let meta = meta.clone();
    //     Callback::from(move |_: MouseEvent|  {
    //         let editor = editor.clone();
    //         let utils = utils.clone();
    //         let url = url.clone();
    //         let meta = meta.clone();
    //         wasm_bindgen_futures::spawn_local(async move {
    //             if let Ok(node) = editor.get_graph_xml() {
    //                 if let Ok(Some(model_str)) = utils.get_xml(node) {
    //                     match meta.model {
    //                         GraphModel::Diagram(_) => {
    //                             let item = ScadaDiagramDto::new("insert proba".to_owned(), model_str);
    //                             post(format!("{url}/diagram"), item).await
    //                                 .and_then(|o| Ok(o.uuid))
    //                                 .map(|pk| {
    //                                     wasm_bindgen_futures::spawn_local(async move {
    //                                         fetch_string(format!("{url}/diagram/{pk}/model")).await
    //                                             .map(|model| {
    //                                                 load_scada_model(&editor, model.as_str());
    //                                             }).unwrap();
    //                                     })
    //                                 })
    //                                 .unwrap();
    //                         },
    //                         GraphModel::Widget(_) => {
    //                             let item = WidgetDto::new("insert proba".to_owned(), model_str);
    //                             post(format!("{url}/widget"), item).await
    //                                 .and_then(|o| Ok(o.uuid))
    //                                 .map(|pk| {
    //                                     wasm_bindgen_futures::spawn_local(async move {
    //                                         fetch_string(format!("{url}/widget/{pk}/model")).await
    //                                             .map(|model| {
    //                                                 load_scada_model(&editor, model.as_str());
    //                                             }).unwrap();
    //                                     })
    //                                 })
    //                                 .unwrap();

    //                         },
    //                     }};
    //                 } 
    //             }
    //         ) 
    //     })
    // };

    // =================== view ====================
    html! {
        <>
            { get_global_css() }        
            <InfoComponent ..(*meta).clone().into() /> 
        </>
    }    
}


#[wasm_bindgen(js_name=renderSchema)]
pub fn render_schema(mx_utils: MxUtils, mx_editor: MxEditor, div: HtmlDivElement, options: SchemaOptions) {
    let props  = Props {
        api_url: options.api_url.unwrap_or("undefiend".to_owned()).into(),
        mx_utils: Rc::new(mx_utils),
        mx_editor: Rc::new(mx_editor),
    };
    yew::Renderer::<App>::with_root_and_props(div.into(), props).render();
    log::info!("schema loaded");
}




