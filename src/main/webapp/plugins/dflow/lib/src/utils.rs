use common_model::diagram::WidgetXml;
use wasm_bindgen::prelude::*;
use web_sys::{js_sys::JsString, Element, HtmlDivElement};

use reqwasm::{
    http::Request,
//    Error
};
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen_futures::wasm_bindgen;

use crate::{errors::FetchError, model::{common::DiagramMeta, mx_cell::{CellValue, MxCell}, mx_editor::MxEditor}};

pub const NULL_UUID: &str = "00000000-0000-0000-0000-000000000000";
pub const NULL_MODEL: &str = "<mxGraphModel/>";
pub const NULL_GLYPH_SVG: &str = "<svg/>";
pub const NULL_GLYPH_SIZED: &str = r#"{"glyph": "PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZlcnNpb249IjEuMSIgd2lkdGg9IjMzcHgiIGhlaWdodD0iMzNweCIgdmlld0JveD0iLTAuNSAtMC41IDMzIDMzIiBzdHlsZT0iYmFja2dyb3VuZC1jb2xvcjogcmdiKDI1NSwgMjU1LCAyNTUpOyI+CiAgICA8Zz4KICAgICAgICA8cmVjdCB4PSIwIiB5PSIwIiB3aWR0aD0iMzIiIGhlaWdodD0iMzIiIGZpbGw9Im5vbmUiIHN0cm9rZT0icmdiKDAsIDAsIDApIi8+CiAgICA8L2c+CiAgICA8ZyBmaWxsPSJyZ2IoMCwgMCwgMCkiIGZvbnQtZmFtaWx5PSImcXVvdDtIZWx2ZXRpY2EmcXVvdDsiIHRleHQtYW5jaG9yPSJtaWRkbGUiIGZvbnQtc2l6ZT0iMTJweCI+CiAgICAgICAgPHRleHQgeD0iMTUuNSIgeT0iMjAuNSI+PzwvdGV4dD4KICAgIDwvZz4KPC9zdmc+Cg==", "geom": {"x":0.0,"y":0.0,"width":33.0,"height":33.0}}"#;


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


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name=loadDFlowModel)]
    pub fn load_dflow_model(editor: &MxEditor, xmlStr: &str);

    #[wasm_bindgen(js_name=getCell0)]
    pub fn get_cell0(editor: &MxEditor) -> MxCell;

    #[wasm_bindgen(js_name=setCell0Value)]
    pub fn set_cell0_value(editor: &MxEditor, value: String);

    #[wasm_bindgen(js_name=getPrettyXml)]
    pub fn get_pretty_xml(el: Element) -> JsString;

    #[wasm_bindgen(js_name=getGraphSvg)]
    pub fn get_graph_svg(editor: &MxEditor) -> JsString;

    // #[wasm_bindgen(js_name=setWidgetModel)]
    // pub fn set_widget_model(editor: &MxEditor, cell: &MxCell, model_str: String);

    #[wasm_bindgen(js_name=setWidgetContainerGlyph)]
    pub fn set_widget_container_glyph(editor: &MxEditor, cell: &MxCell, model_str: String);

    #[wasm_bindgen(js_name=clipedModelBox)]
    pub fn cliped_model_box(model_str: String) -> JsString;

    #[wasm_bindgen(js_name=recreateWidgetModelInfo)]
    pub fn recreate_widget_model_info(editor: &MxEditor, modelStr: String, recreateFun: &Closure<dyn Fn(JsValue)>);

    #[wasm_bindgen(js_name=recreateDiagramModelInfo)]
    pub fn recreate_diagram_model_info(editor: &MxEditor, modelStr: String, recreateFun: &Closure<dyn Fn(JsValue)>);

    #[wasm_bindgen(js_name=recreateCellInfo)]
    pub fn recreate_cell_info(recreateFun: &Closure<dyn Fn(JsValue)>);

    // #[wasm_bindgen(js_name=setDefaultStyles)]
    // pub fn set_default_styles(editor: &MxEditor);

    #[wasm_bindgen(js_name=refreshCell)]
    pub fn refresh_cell(editor: &MxEditor, cell: &MxCell);

    // #[wasm_bindgen(js_name=encodeURIComponent)]
    // pub fn encode_uri_component(uri: &str) -> JsString;
}

//------------------------------------------------------------------
pub async fn fetch<T>(url: String) -> Result<T, FetchError>
where
    T: DeserializeOwned,
{
    return Request::get(&url)
        .send().await
        .map_err(|err| FetchError::RequestError(err.to_string()))?
        .json::<T>().await
        .map_err(|err| FetchError::SerdeError(err.to_string()));
}

pub async fn fetch_string(url: String) -> Result<String, FetchError>
{
    return Request::get(&url).send().await
        .map_err(|err| FetchError::RequestError(err.to_string()))?
        .text().await
        .map_err(|err| FetchError::RequestError(err.to_string()));
}

pub async fn post<T>(url: String, data: T) -> Result<T, FetchError>
where
    T: Serialize,
    T: DeserializeOwned,
{
    let json = serde_json::to_string(&data)
        .map_err(|err| FetchError::SerdeError(err.to_string()))?;

    return Request::post(&url)
        .header("Content-Type", "application/json")
        .body(json)
        .send().await
        .map_err(|err| FetchError::RequestError(err.to_string()))?
        .json::<T>().await
        .map_err(|err| FetchError::SerdeError(err.to_string()));
}

pub async fn put<T>(url: String, data: T) -> Result<T, FetchError>
where
    T: Serialize,
    T: DeserializeOwned,
{
    let json = serde_json::to_string(&data)
        .map_err(|err| FetchError::SerdeError(err.to_string()))?;

    return Request::put(&url)
        .header("Content-Type", "application/json")
        .body(json)
        .send().await
        .map_err(|err| FetchError::RequestError(err.to_string()))?
        .json::<T>().await
        .map_err(|err| FetchError::SerdeError(err.to_string()));
}

/**
 * returns  Option<(cell0.value.outer_html, WidgetXml)>
 */
pub fn get_cell0_meta(editor: &MxEditor) -> Option<DiagramMeta> {
    if let Ok(CellValue::Object(el)) = get_cell0(editor).get_value() {
        if let Ok(meta) = quick_xml::de::from_str::<DiagramMeta>(el.outer_html().as_str()) {
            return Some(meta);
        }
    }

    log::error!("bad CellValue: {:?}", get_cell0(editor).get_value().unwrap());

    // else
    None
}