use std::str::FromStr;

use common_model::{
    data_source::DataSourceXml, dflow_cell::DFlowVariant, geom_value::GeomValueXml, label_value::LabelValueXml,
    multystate::{
        range::{RangeType, RangeValue}, state::StateXml, state_predef::{PredefStateXml, StatePredefXml}, MultystateXml
    }, widget::{self, WidgetContainerXml}
};
use web_sys::FormData;
use yew::AttrValue;

use crate::{
    components::multystate::{
        FORM_NAME_PREFIX, FORM_NAME_SUFIX_FROM,
        FORM_NAME_SUFIX_NAME, FORM_NAME_SUFIX_PK,
        FORM_NAME_SUFIX_STYLE, FORM_NAME_SUFIX_VALUE, RANGE_TYPE
    },
    model::cell_meta::{CELL_TYPE_GEOM, CELL_TYPE_LABEL, CELL_TYPE_MULTY, CELL_TYPE_WIDGET_CONTAINER}
};


#[derive(Debug, PartialEq, Clone, Default)]
pub struct CellDetailsForm {
    pub variants: Vec<DFlowVariant>,
    pub widget_model: Option<AttrValue>,
}


impl From<FormData> for CellDetailsForm {
    fn from(data: FormData) -> Self {
        let mut variants  = Vec::<DFlowVariant>::new();

        // work with label
        let target = CELL_TYPE_LABEL;
        if data.has(format!("{target}:formGroup").as_str()) {
            let ds = get_formdata_data_source(&data, target, Default::default());

            let meta = LabelValueXml { ds };

            variants.push(DFlowVariant::Label(meta));
        }

        // work with multystate
        let target = CELL_TYPE_MULTY;
        if data.has(format!("{target}:formGroup").as_str()) {
            let ds = get_formdata_data_source(&data, target, Default::default());

            let mut predef_styles = Vec::<StatePredefXml>::new();
            if let Some(style) = get_formdata_string_value(&data, target, "style-0") {
                predef_styles.push( StatePredefXml::Default( PredefStateXml { style: style.into() }) );
            }
            if let Some(style) = get_formdata_string_value(&data, target, "style-1") {
                predef_styles.push( StatePredefXml::Bad( PredefStateXml { style: style.into() }) );
            }

            let mut meta = MultystateXml {
                ds,
                predef: predef_styles,
                ..Default::default()
            };

            // get arrays of inputs from formdata
            let state_pk = get_formdata_all_string_values(&data, target, &format!("{FORM_NAME_PREFIX}-{FORM_NAME_SUFIX_PK}"));
            let state_name = get_formdata_all_string_values(&data, target, &format!("{FORM_NAME_PREFIX}-{FORM_NAME_SUFIX_NAME}"));
            let state_style = get_formdata_all_string_values(&data, target, &format!("{FORM_NAME_PREFIX}-{FORM_NAME_SUFIX_STYLE}"));

            // what is range type
            let range_type = get_formdata_string_value(&data, target, RANGE_TYPE)
                .map(|o| RangeType::from(o));

            if let Some(range_type) = range_type {
                meta.range_type = range_type.clone();

                match range_type {
                    RangeType::DISCRET => {
                        let state_value = get_formdata_all_string_values(&data, target, &format!("{FORM_NAME_PREFIX}-{FORM_NAME_SUFIX_VALUE}"));

                        let states = state_pk.into_iter()
                            .zip(state_name.into_iter())
                            .zip(state_value.into_iter())
                            .zip(state_style.into_iter())
                            .map(|(((pk, name), value), style)| {
                                // what is RangeValue?
                                let range_value = match value.parse::<u32>() {
                                        Ok(value) => RangeValue::DiscretConst { value },
                                        Err(_) => RangeValue::DiscretTag { value },
                                    };

                                // result
                                StateXml {
                                    pk: usize::from_str(&pk).unwrap_or_default(),
                                    name: name.into(),
                                    style: style.into(),
                                    value: range_value,
                                }
                            })
                            .collect::<Vec<_>>();

                        meta.states = states;
                    },
                    RangeType::RANGE => {
                        let state_from = get_formdata_all_string_values(&data, target, &format!("{FORM_NAME_PREFIX}-{FORM_NAME_SUFIX_FROM}"));

                        let states = state_pk.into_iter()
                            .zip(state_name.into_iter())
                            .zip(state_from.into_iter())
                            .zip(state_style.into_iter())
                            .map(|(((pk, name), from), style)| {
                                // what is RangeValue?
                                let range_value = match from.parse::<f32>() {
                                        Ok(from) => RangeValue::RangeConst { from },
                                        Err(_) => RangeValue::RangeTag { from },
                                    };

                                // result
                                StateXml {
                                    pk: usize::from_str(&pk).unwrap_or_default(),
                                    name: name.into(),
                                    style: style.into(),
                                    value: range_value,
                                }
                            })
                            .collect::<Vec<_>>();

                        meta.states = states;
                    },
                }
            }

            variants.push(DFlowVariant::Multystate(meta));
        }

        // work with geometry
        let target = CELL_TYPE_GEOM;
        if data.has(format!("{target}:formGroup").as_str()) {
            let ds = get_formdata_data_source(&data, target, Default::default());
            let min = get_formdata_typed_value::<f32>(&data, target, "min");
            let max = get_formdata_typed_value::<f32>(&data, target, "max");

            let meta = GeomValueXml {
                    min: min.unwrap_or_default(),
                    max: max.unwrap_or_default(),
                    ds,
                    ..Default::default()
                };

            variants.push(DFlowVariant::Geometry(meta));
        }

        // work with widget container
        let mut widget_model: Option<AttrValue> = None;
        let target = CELL_TYPE_WIDGET_CONTAINER;
        if data.has(format!("{target}:formGroup").as_str()) {
            let ds = get_formdata_data_source(&data, target, Default::default());
            let uuid = get_formdata_string_value(&data, target, "uuid").expect("uuid must be");
            let group = get_formdata_string_value(&data, target, "group").expect("group must be");
            widget_model = Some( get_formdata_string_value(&data, target, "model").expect("model must be").into() );

            log::debug!("widget_model: {widget_model:?}");

            let meta = WidgetContainerXml {
                    uuid: uuid.into(),
                    group: group.into(),
                    ds,
                };

            variants.push(DFlowVariant::WidgetContainer(meta));
        }

        // result
        Self {
            variants,
            widget_model,
        }
    }
}

fn get_formdata_string_value(data: &FormData, target: &str, name: &str) -> Option<String> {
    data.get(format!("{target}:{name}").as_str()).as_string()
}

fn get_formdata_all_string_values(data: &FormData, target: &str, name: &str) -> Vec<String> {
    data.get_all(format!("{target}:{name}").as_str()).into_iter()
        .map(|o| o.as_string())
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect::<Vec<_>>()
}

fn get_formdata_typed_value<T: FromStr>(data: &FormData, target: &str, name: &str) -> Option<T> {
    let value = get_formdata_string_value(data, target, name);
    if let Some(value) = value {
        return match value.parse::<T>() {
                Ok(value) => Some(value),
                Err(_) => { log::error!("parse error for {value}"); None },
            };
    }
    // result
    None
}

fn get_formdata_data_source(data: &FormData, target: &str, initial: DataSourceXml) -> DataSourceXml {
    match get_formdata_string_value(data, target, "tag") {
        Some(tag) => DataSourceXml {
            tag: tag.into(),
            ..initial
        },
        None => initial
    }
}