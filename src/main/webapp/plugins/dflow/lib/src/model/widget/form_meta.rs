use common_model::{data_source::DataSourceXml, diagram::WidgetPropertyXml};
use web_sys::FormData;
use yew::AttrValue;

use crate::model::common::{DiagramMeta, GraphModel};

use super::NULL_UUID;

#[derive(Debug, PartialEq, Clone)]
pub struct WidgetForm {
    pub uuid: AttrValue,
    pub name: AttrValue,
    pub group: AttrValue,
    pub meta: AttrValue,
}

impl WidgetForm {
    pub fn is_new_item(&self) -> bool {
        self.uuid == NULL_UUID
    }
}

impl Default for WidgetForm {
    fn default() -> Self {
        Self {
            uuid: NULL_UUID.into(),
            name: Default::default(),
            group: Default::default(),
            meta: Default::default(),
        }
    }
}

impl From<FormData> for WidgetForm {
    fn from(data: FormData) -> Self {
        let meta = data.get("meta").as_string().unwrap();  // this is current cell0 value
        let mut meta_str = meta.clone();

        log::debug!("meta_str {meta_str}");

        match quick_xml::de::from_str::<DiagramMeta>(&meta) {
            Ok(meta) => {
                let prop_names = data.get_all("props-name").into_iter()
                    .map(|o| o.as_string().unwrap())
                    .collect::<Vec<_>>();

                let prop_values = data.get_all("props-value").into_iter()
                    .map(|o| o.as_string().unwrap())
                    .collect::<Vec<_>>();

                let props = prop_names.into_iter().zip(prop_values.into_iter())
                    .map(|o| {
                        WidgetPropertyXml {
                            name:  o.0,
                            ds: DataSourceXml { tag: o.1.into(), ..Default::default() },
                        }
                    })
                    .collect::<Vec<_>>();

                if let GraphModel::Widget(mut widget) = meta.model {
                    widget.property = props;

                    let new_meta = DiagramMeta {
                            label: meta.label,
                            model: GraphModel::Widget(widget),
                        };

                    meta_str = quick_xml::se::to_string(&new_meta).unwrap();
                    log::debug!("meta_str {:?}", meta_str);
                }
            },
            Err(err) => log::error!("{err}"),
        }

        Self {
            uuid: data.get("uuid").as_string().unwrap_or_default().into(),
            name: data.get("name").as_string().unwrap_or_default().into(),
            group: data.get("group").as_string().unwrap_or_default().into(),
            meta: meta_str.into(),
        }
    }
}


// ==========================================================
#[cfg(test)]
mod tests {




}