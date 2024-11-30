use common_model::{data_source::DataSourceXml, dflow_cell::DFlowVariant, label_value::LabelValueXml};
use yew::{function_component, html, use_effect_with, use_state, Callback, Html, Properties };
use yew_hooks::use_unmount;
use yewdux::use_selector;

use crate::{
    components::{data_source::{self, DataSource}, shared::use_my_datasource}, 
    store::cell,
};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub edit_mode: bool,
    pub value: LabelValueXml,
    pub on_detals_apply: Callback<DFlowVariant>,
}

#[function_component]
pub fn LabelValueComponent(Props {
    edit_mode, 
    value,
    on_detals_apply,
}: &Props ) -> Html 
{
    use_unmount(|| {
        log::debug!("LabelValueComponent unmount");
    });    
    
    let data_source = use_my_datasource(value.clone());

    let start_apply = use_selector(|state: &cell::State | state.start_apply);
    {    
        // let label_state = label_state.clone();
        let on_detals_apply = on_detals_apply.clone();
        let data_source = data_source.clone();
        use_effect_with(*start_apply, move |start| {
            if *start {
                let new_label = LabelValueXml { ds: (*data_source).clone() };

                log::debug!("NEW LABEL {new_label:?}");

                let new_variant = DFlowVariant::Label(new_label);
                on_detals_apply.emit(new_variant);
            }
        })
    };

    let apply_ds = {
            let data_source = data_source.clone();
            Callback::from(move |ds: DataSourceXml| {
                data_source.set(ds);
            })
        };        

    let data_source_view = {
        let data_source = data_source.clone();
        let apply_ds = apply_ds.clone();
        let props = yew::props!(data_source::Props {
            ds: (*data_source).clone(),
            edit_mode: *edit_mode,
            on_apply: apply_ds,
        });
        html! {<DataSource ..props/>}
    };


    html!{
        <fieldset>
            <legend>{"Настройки значения:"}</legend>
           { data_source_view }
        </fieldset>
    }
    
}