use wasm_bindgen::prelude::*;
use web_sys::{js_sys::JsString, Element};
use std::collections::HashMap;

use reqwasm::{
    http::Request, 
//    Error
};
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen_futures::wasm_bindgen;

use crate::{errors::FetchError, model::{mx_cell::MxCell, mx_editor::MxEditor}};

pub const NULL_UUID: &str = "00000000-0000-0000-0000-000000000000";

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
    #[wasm_bindgen(js_name=loadScadaModel)]
    pub fn load_scada_model(editor: &MxEditor, xmlStr: &str);

    #[wasm_bindgen(js_name=getCell0)]
    pub fn get_cell0(editor: &MxEditor) -> MxCell;

    #[wasm_bindgen(js_name=getPrettyXml)]
    pub fn get_pretty_xml(el: Element) -> JsString;
    
    #[wasm_bindgen(js_name=getGraphSvg)]
    pub fn get_graph_svg(editor: &MxEditor) -> JsString;    
    
    #[wasm_bindgen(js_name=setWidgetModel)]
    pub fn set_widget_model(editor: MxEditor, cell: MxCell, model_str: String);

    #[wasm_bindgen(js_name=clipedModelBox)]
    pub fn cliped_model_box(model_str: String) -> JsString;

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


pub fn string_to_map<'a>(s: &'a str) -> HashMap<&'a str, &'a str> {
    s.split(';')
        .map(|o| o.trim())
        .map(|o| {
            o.split(':')
                .map(|p| p.trim())
                .filter(|s| s.len() > 0)
                .collect::<Vec<_>>()
        })
        .filter(|v| v.len() == 2)
        .map(|kv| (kv[0], kv[1]))
        .fold(HashMap::new(), |mut acc, i| {
            acc.insert(i.0, i.1);
            acc
        })
}

pub fn map_to_string<'a>(m: HashMap<&'a str, &'a str>) -> String {
    m.iter()
        .map(|o| format!("{}:{}", o.0, o.1))
        .collect::<Vec<_>>()
        .join(";")
}

pub fn mx_style_to_map<'a>(s: &'a str) -> HashMap<&'a str, &'a str> {
    s.split(';')
        .map(|o| o.trim())
        .map(|o| {
            o.split('=')
                .map(|p| p.trim())
                .filter(|s| s.len() > 0)
                .collect::<Vec<_>>()
        })
        .filter(|v| v.len() == 2)
        .map(|kv| (kv[0], kv[1]))
        .fold(HashMap::new(), |mut acc, i| {
            acc.insert(i.0, i.1);
            acc
        })
}

pub fn map_to_mx_style<'a>(m: HashMap<&'a str, &'a str>) -> String {
    m.iter()
        .map(|o| format!("{}={}", o.0, o.1))
        .collect::<Vec<_>>()
        .join(";")
}

pub fn map_to_svg_style<'a>(m: HashMap<&'a str, &'a str>) -> String {
    let fill = m.get(&"fillColor").unwrap_or(&"black");
    let stroke = m.get(&"strokeColor").unwrap_or(&"black");

    let stroke_width = m.get(&"strokeWidth")
        .map(|w| format!("stroke-width:{w}"))
        .unwrap_or("".to_owned());

    format!("fill:{fill};stroke:{stroke};{stroke_width}")
}


// ==========================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_map_works() {
        let str = " aaa: bbb; ccc:ddd;";

        let map = string_to_map(str);
        println!("{map:#?}");

        assert_eq!(map.get("aaa"), Some(&"bbb"));

        let res = map_to_string(map);
        println!("{res:#?}");

        // assert_eq!(res, "ccc:ddd;aaa:bbb");
    }
}