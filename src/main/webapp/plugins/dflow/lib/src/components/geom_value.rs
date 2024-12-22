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

    html! {
        if *edit_mode {
            <div class="datails-panel">
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
                }
            </div>
        }
        else {
            <div class="datails-panel">
                <div class="input-valign-center">{ "Настройки высоты:" }</div>
                <div>
                    <table class="prop-table">
                        <tr>
                            <td class="label">{ "тег:" }</td>
                            <td>{ meta.ds.tag.clone() }</td>
                        </tr>
                        <tr>
                            <td class="label">{ "min:" }</td>
                            <td>{ meta.min }</td>
                        </tr>
                        <tr>
                            <td class="label">{ "max:" }</td>
                            <td>{ meta.max }</td>
                        </tr>
                    </table>
                </div>
            </div>
        }
    }
}
