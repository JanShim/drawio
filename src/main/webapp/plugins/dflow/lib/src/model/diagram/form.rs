use implicit_clone::unsync::IString;
use web_sys::FormData;

use crate::model::common::{DiagramMeta, GraphModel};

use super::NULL_UUID;

#[derive(Debug, PartialEq, Clone)]
pub struct DiagramForm {
    pub uuid: IString,
    pub name: IString,
    pub diagram_meta: DiagramMeta,     // this.is from cell0
}

impl DiagramForm {
    pub fn is_new_item(&self) -> bool {
        self.uuid == NULL_UUID
    }
}

impl Default for DiagramForm {
    fn default() -> Self {
        Self {
            uuid: NULL_UUID.into(),
            name: Default::default(),
            diagram_meta: DiagramMeta::get_diagram_default(),
        }
    }
}

impl From<FormData> for DiagramForm {
    fn from(data: FormData) -> Self {
        let meta = data.get("meta").as_string().unwrap();  // this is current cell0 value

        log::debug!("IN From<FormData> meta_str {meta}");

        let mut ret = Self {
                uuid: data.get("uuid").as_string().unwrap_or_default().into(),
                name: data.get("name").as_string().unwrap_or_default().into(),
                ..Default::default()
            };

        match quick_xml::de::from_str::<DiagramMeta>(&meta) {
            Ok(meta) => {
                if let GraphModel::Diagram(diagram) = meta.model {
                    // set new meta data
                    ret.diagram_meta = DiagramMeta { model: GraphModel::Diagram(diagram), ..meta };

                    log::debug!("OUT From<FormData> DiagramForm:: {ret:?}");
                    return ret;
                }
            },
            Err(err) => log::error!("{err}"),
        }

        // result
        ret
    }
}

// ==========================================================
#[cfg(test)]
mod tests {
    // use mockall::mock;
    // use wasm_bindgen::JsValue;
    // use web_sys::FormData;

    // use super::DiagramForm;

    // pub trait A {
    //     fn get(&self, name: JsValue) -> JsValue;
    // }

    // mock! {
    //     FormData {}     // Name of the mock struct, less the "Mock" prefix
    //     impl A for FormData {   // specification of the trait to mock
    //         fn get(&self, name: JsValue) -> JsValue;
    //     }
    // }

    // #[test]
    // fn to_diagram_form_works() {

    //     let mut mock_form_data = MockFormData::new();

    //     mock_form_data.expect_get()
    //         .returning(|t| t);

    //     let form  = Into::<DiagramForm>::into(mock_form_data);
    // }

}