use common_model::{data_source::DataSourceXml, dflow_cell::DFlowVariant, label_value::LabelValueXml};
use yew::{classes, function_component, html, use_effect_with, use_memo, use_state, Callback, Event, Html, MouseEvent, Properties };
use yew_hooks::use_unmount;
use yewdux::use_selector;

use crate::{
    components::{data_source::{self, DataSource}, shared::use_my_datasource},
    store::cell,
};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub edit_mode: bool,
    pub value: Option<LabelValueXml>,
    // pub on_detals_apply: Callback<DFlowVariant>,
}

#[function_component]
pub fn LabelValueComponent(Props {
    edit_mode,
    value,
    // on_detals_apply,
}: &Props ) -> Html
{
    use_unmount(|| {
        log::debug!("LabelValueComponent unmount");
    });

    let meta = use_state(|| {
            match value {
                Some(value) => value.clone(),
                None => LabelValueXml:: default(),
            }
        });

    let checked = use_state(|| {
            value.is_some()
        });

    // let class_disabled = use_memo(*checked, | checked | {
    //         match checked { true => None, false => Some("disabled") }
    //     });

    let on_checked_change = {
            let checked = checked.clone();
            Callback::from(move |_: Event| {
                checked.set(!*checked);
            })
        };

    // let data_source = use_my_datasource(value.clone());

    // let start_apply = use_selector(|state: &cell::State | state.start_apply);
    // {
    //     // let label_state = label_state.clone();
    //     let on_detals_apply = on_detals_apply.clone();
    //     let data_source = data_source.clone();
    //     use_effect_with(*start_apply, move |start| {
    //         if *start {
    //             let new_label = LabelValueXml { ds: (*data_source).clone() };

    //             log::debug!("NEW LABEL {new_label:?}");

    //             let new_variant = DFlowVariant::Label(new_label);
    //             on_detals_apply.emit(new_variant);
    //         }
    //     })
    // };

    // let apply_ds = {
    //         let data_source = data_source.clone();
    //         Callback::from(move |ds: DataSourceXml| {
    //             data_source.set(ds);
    //         })
    //     };

    // let data_source_view = {
    //     let data_source = data_source.clone();
    //     let apply_ds = apply_ds.clone();
    //     let props = yew::props!(data_source::Props {
    //         ds: (*data_source).clone(),
    //         edit_mode: *edit_mode,
    //         on_apply: apply_ds,
    //     });
    //     html! {<DataSource ..props/>}
    // };



    html!{
        if *edit_mode {
            <div class="datails-panel">
                <div class="input-valign-center">
                    <input type="checkbox" id="label" name="label" checked={*checked} onchange={on_checked_change}/>
                    <label for="label">{"Настройки значения:"}</label>
                </div>
                <div>
                    // <pre>{ format!("{meta:?}") }</pre>
                    <table class="prop-table">
                    <tr>
                        <td>{"тег:"}</td>
                        <td width="100%">
                            <input id="label-tag" name="label-tag" class="input-100"
                                disabled={ !*checked }
                                value={ "" }
                            />
                        </td>
                    </tr>
                    </table>
                </div>
            </div>
        }

        // <fieldset>
        //     <legend>{"Настройки значения:"}</legend>
        // //    { data_source_view }

        //     <pre>{ format!("{value:?}") }</pre>
        // </fieldset>
    }

}