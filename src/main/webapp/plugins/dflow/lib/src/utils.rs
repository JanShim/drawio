use common_model::diagram::WidgetXml;
use wasm_bindgen::prelude::*;
use web_sys::{js_sys::JsString, Element};

use reqwasm::{
    http::Request,
//    Error
};
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen_futures::wasm_bindgen;
use yew::AttrValue;

use crate::{errors::FetchError, model::{mx_cell::{CellValue, MxCell}, mx_editor::MxEditor, widget::form_meta::WidgetForm}};

pub const NULL_UUID: &str = "00000000-0000-0000-0000-000000000000";
pub const NULL_MODEL: &str = "<mxGraphModel/>";
pub const NULL_GLYPH: &str = "<svg/>";


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

    #[wasm_bindgen(js_name=setWidgetModel)]
    pub fn set_widget_model(editor: MxEditor, cell: MxCell, model_str: String);

    #[wasm_bindgen(js_name=clipedModelBox)]
    pub fn cliped_model_box(model_str: String) -> JsString;

    // #[wasm_bindgen(js_name=getDiagramBoundingBox)]
    // pub fn get_diagram_bounding_box(editor: &MxEditor) -> JsValue;

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
pub fn get_cell0_widget_meta(editor: &MxEditor) -> Option<(AttrValue, WidgetXml)> {
    if let Ok(CellValue::Object(el)) = get_cell0(editor).get_value() {
        if let Ok(widget_xml) = quick_xml::de::from_str::<WidgetXml>(el.inner_html().as_str()) {
            let outer_html = el.outer_html();
            return Some((outer_html.into(), widget_xml));
        }
    }

    log::error!("bad CellValue: {:?}", get_cell0(editor).get_value().unwrap());

    // else
    None
}