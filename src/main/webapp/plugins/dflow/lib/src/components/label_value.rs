use common_model::label_value::LabelValueXml;
use yew::{function_component, html, use_memo, use_state, AttrValue, Html, Properties };
use yew_hooks::use_unmount;

use crate::model::cell_meta::CELL_TYPE_LABEL;
use crate::components::{
    prop_table_tr::PropTableTr,
    shared::{use_checked, InputType}
};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub edit_mode: bool,
    pub value: Option<LabelValueXml>,
}

#[function_component]
pub fn LabelValueComponent(Props {
    edit_mode,
    value,
}: &Props ) -> Html
{
    use_unmount(|| {
        log::debug!("LabelValueComponent unmount");
    });

    let meta = use_memo(value.clone(), |value| {
        match value {
            Some(value) => value.clone(),
            None => LabelValueXml::default(),
        }
    });

    let (checked, on_checked_toggle) = use_checked(value.is_some());

    // ============= view ==================
    let props_table = html! {
            <div>
                <table class="prop-table">
                    <PropTableTr<AttrValue>
                        { edit_mode }
                        checked={ *checked }
                        name={ format!("{CELL_TYPE_LABEL}:tag") }
                        label={ "тег:" }
                        value={ meta.ds.tag.clone() }
                        value_type={InputType::STRING}
                    />
                </table>
            </div>
        };

    html!{
        <div class="datails-panel">
            if *edit_mode {
                <div class="input-valign-center">
                    if *checked {
                        <input type="hidden"
                            id={ format!("{CELL_TYPE_LABEL}:formGroup") }
                            name={ format!("{CELL_TYPE_LABEL}:formGroup") }
                        />
                    }
                    <input type="checkbox" id="label" name="label" checked={*checked} onchange={on_checked_toggle}/>
                    <label for="label">{ "Настройки значения:" }</label>
                </div>
                if *checked {
                    { props_table }
                }
            } else {
                <div class="input-valign-center">{ "Настройки значения:" }</div>
                { props_table }
            }
        </div>
    }

}