use state::MultystateStateEditComponent;
use states::StatesSelector;
use yew::prelude::*;
use yew_hooks::{use_list, use_unmount};
use common_model::multystate::{range::RangeType, state_predef::StatePredefXml, MultystateXml};
use state_predef::StatePredefEditComponent;

use crate::components::{prop_table_tr::PropTableTr, shared::InputType};
use crate::model::cell_meta::CELL_TYPE_MULTY;
use crate::components::shared::use_checked;

// pub mod type_selector;
pub mod states;
pub mod state;
pub mod state_rect;
pub mod state_predef;

pub const FORM_NAME_PREFIX: &str = "state";
pub const FORM_NAME_SUFIX_PK: &str = "pk";
pub const FORM_NAME_SUFIX_NAME: &str = "name";
pub const FORM_NAME_SUFIX_VALUE: &str = "value";
pub const FORM_NAME_SUFIX_FROM: &str = "from";
pub const FORM_NAME_SUFIX_STYLE: &str = "style";
pub const RANGE_TYPE: &str = "range-type";

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub edit_mode: bool,
    pub value: Option<MultystateXml>,
}

#[function_component]
pub fn MultystateComponent(Props {
    edit_mode ,
    value,
}: &Props) -> Html
{
    use_unmount(|| {
        log::debug!("MultystateComponent unmount");
    });

    let meta = use_memo(value.clone(), |value| {

        log::debug!("initial meta: {value:?}");

        match value {
            Some(value) => value.clone(),
            None => MultystateXml::default(),
        }
    });

    let (checked, on_checked_toggle) = use_checked(value.is_some());

    let range_type = use_state(|| meta.range_type.clone());

    let predef_states = use_state(|| meta.predef.clone());

    let states = use_list(meta.states.clone());

    // ======== Events ==========
    let on_range_type_change = {
            let range_type_handler = range_type.clone();
            let states = states.clone();
            Callback::from(move |range_type: RangeType| {
                states.clear();
                range_type_handler.set(range_type)
            })
        };

    // //====== View Items =====
    let states_view = {
            let edit_mode = edit_mode.clone();
            states.current().iter()
                .map(move |item| {
                    html! {
                        <MultystateStateEditComponent
                            { edit_mode }
                            value={ (*item).clone() }
                        />
                    }
                })
                .collect::<Vec<_>>()
        };


    // ============= view ==================
    let props_table = html! {
            <div>
                <table class="prop-table">
                    <PropTableTr<AttrValue>
                        { edit_mode }
                        checked={ *checked }
                        name={ format!("{CELL_TYPE_MULTY}:tag") }
                        label={ "тег:" }
                        value={ meta.ds.tag.clone() }
                        value_type={InputType::STRING}
                    />

                    <StatePredefEditComponent<StatePredefXml>
                        { edit_mode }
                        checked={ *checked }
                        name={ format!("{CELL_TYPE_MULTY}:style-0") }
                        value={ (*predef_states)[0].clone() }
                    />
                    <StatePredefEditComponent<StatePredefXml>
                        { edit_mode }
                        checked={ *checked }
                        name={ format!("{CELL_TYPE_MULTY}:style-1") }
                        value={ (*predef_states)[1].clone() }
                    />

                    <StatesSelector
                        { edit_mode }
                        states={ states.clone() }
                        range_type={ (*range_type).clone() }
                        {on_range_type_change}
                    />
                    { states_view }

                </table>
            </div>
        };

    html!{
        <div class="datails-panel">
            if *edit_mode {
                <div class="input-valign-center">
                    if *checked {
                        <input type="hidden"
                            id={ format!("{CELL_TYPE_MULTY}:formGroup") }
                            name={ format!("{CELL_TYPE_MULTY}:formGroup") }
                        />
                        <input type="hidden"
                            id={ format!("{CELL_TYPE_MULTY}:{RANGE_TYPE}") }
                            name={ format!("{CELL_TYPE_MULTY}:{RANGE_TYPE}") }
                            value={ (*range_type).to_string() }
                        />
                    }
                    <input type="checkbox" id="label" name="label" checked={*checked} onchange={on_checked_toggle}/>
                    <label for="label">{ "Множественные состояния:" }</label>
                </div>

                if *checked {
                    { props_table }
                }
            } else {
                <div class="input-valign-center">{ "Множественные состояния:" }</div>
                { props_table }
            }
        </div>
    }
}
