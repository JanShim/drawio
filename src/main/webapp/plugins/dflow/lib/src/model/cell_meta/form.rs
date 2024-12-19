use web_sys::FormData;
use yew::AttrValue;




#[derive(Debug, PartialEq, Clone, Default)]
pub struct CellDetailsForm {
    pub label_tag: Option<AttrValue>,
    // pub name: AttrValue,
    // pub diagram_meta: DiagramMeta,
}


impl From<FormData> for CellDetailsForm {
    fn from(data: FormData) -> Self {
        let label_tag = data.get("label-tag").as_string();

        let form = Self {
                label_tag: label_tag.map(|o| o.into()),
                ..Default::default()
            };

        log::debug!("IN From<FormData> data {form:?}");


        // result
        Default::default()


        // let mut ret = Self {
        //         uuid: data.get("uuid").as_string().unwrap_or_default().into(),
        //         name: data.get("name").as_string().unwrap_or_default().into(),
        //         ..Default::default()
        //     };

        // match quick_xml::de::from_str::<DiagramMeta>(&meta) {
        //     Ok(meta) => {
        //         if let GraphModel::Diagram(diagram) = meta.model {
        //             // set new meta data
        //             ret.diagram_meta = DiagramMeta { model: GraphModel::Diagram(diagram), ..meta };

        //             log::debug!("OUT From<FormData> DiagramForm:: {ret:?}");
        //             return ret;
        //         }
        //     },
        //     Err(err) => log::error!("{err}"),
        // }

        // // result
        // ret
    }
}