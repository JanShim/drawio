use yew::{function_component, html, use_memo, AttrValue, Html, Properties };
use yew_hooks::use_unmount;
use common_model::geom_value::GeomValueXml;

use crate::model::cell_meta::CELL_TYPE_GEOM;
use crate::components::{ prop_table_tr::PropTableTr, shared::{use_checked, InputType} };

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub edit_mode: bool,
    pub value: Option<GeomValueXml>,
}

#[function_component]
pub fn GeomValue(Props {
    edit_mode,
    value,
}: &Props ) -> Html
{
    use_unmount(|| {
        log::debug!("GeomValue unmount");
    });

    let meta = use_memo(value.clone(), |value| {
        match value {
            Some(value) => value.clone(),
            None => GeomValueXml::default(),
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
                        name={ format!("{CELL_TYPE_GEOM}:tag") }
                        label={ "тег:" }
                        value={ meta.ds.tag.clone() }
                        value_type={InputType::STRING}
                    />
                    <PropTableTr<f32>
                        { edit_mode }
                        checked={ *checked }
                        name={ format!("{CELL_TYPE_GEOM}:min") }
                        label={ "min:" }
                        value={ meta.min }
                        value_type={InputType::NUMBER}
                    />
                    <PropTableTr<f32>
                        { edit_mode }
                        checked={ *checked }
                        name={ format!("{CELL_TYPE_GEOM}:max") }
                        label={ "max:" }
                        value={ meta.max }
                        value_type={InputType::NUMBER}
                    />
                </table>
            </div>
        };

    html! {
        <div class="datails-panel">
            if *edit_mode {
                <div class="input-valign-center">
                    if *checked {
                        <input type="hidden"
                            id={ format!("{CELL_TYPE_GEOM}:formGroup") }
                            name={ format!("{CELL_TYPE_GEOM}:formGroup") }
                        />
                    }
                    <input type="checkbox" id="label" name="label" checked={*checked} onchange={on_checked_toggle}/>
                    <label for="label">{ "Настройки высоты:" }</label>
                </div>
                if *checked {
                    { props_table }
                }
            } else {
                <div class="input-valign-center">{ "Настройки высоты:" }</div>
                { props_table }
            }
        </div>
    }
}
