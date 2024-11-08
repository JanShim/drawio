use common_model::multystate::{range::{RangeType, RangeValue}, state::StateXml, state_predef::StatePredefXml, MultystateXml};
use implicit_clone::unsync::IString;
use state_predef::{StatePredefComponent, StatePredefEditComponent};
use yew::{function_component, html, use_effect_with, use_state, Callback, Html, Properties};
use yew_hooks::use_list;
use yewdux::{use_selector, use_store};

use data_source::DataSourceComponent;
use state::{MultystateStateComponent, MultystateStateEditComponent};

use crate::{
    errors::CellStateError, rrefcell, store::cell::{self, SetMultystateAction},
};

pub mod data_source;
pub mod state;
pub mod state_rect;
pub mod state_predef;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub edit_mode: bool,
    pub on_detals_apply: Callback<bool>,    // callback for applyed notification
}

#[function_component]
pub fn MultystateComponent(Props { edit_mode , on_detals_apply}: &Props) -> Html {
    let (_, store_state_dispatch) = use_store::<cell::State>();
    let cell_state = use_selector(|cell_state: &cell::State| {
        if let Ok(multystate) = cell_state.meta.get_multystate_meta() {
			return multystate;
		};
        log::error!("{}", CellStateError::NotMultystate);
        MultystateXml::default()
    });    

    let predef_states = use_state(|| cell_state.predef.clone());
    let states = use_list(cell_state.states.clone());

    /* #region selected_state */
    let selected_state = use_state(|| {
        let value: Option<StateXml> = None;
        value
    });

    let state_select_callback = {
        let selected = selected_state.clone();
        Callback::from(move |value: Option<StateXml>| {
            log::debug!("state_select_callback: {value:?}");

            selected.set(value);  // change selected
        })
    };
    /* #endregion */

    // start apply process if true
    let start_apply = use_selector(|state: &cell::State | state.start_apply);
    {    
        let my_state = cell_state.clone();
        let on_detals_apply = on_detals_apply.clone();
        let predef_states = predef_states.clone();
        let states = states.clone();
        use_effect_with(*start_apply, move |start| {
            if *start {
                let new_state = MultystateXml {
                    predef: (*predef_states).clone(),
                    states: states.current().clone(),
                    ..(*my_state).clone()
                };

                store_state_dispatch.apply(SetMultystateAction(new_state));
                on_detals_apply.emit(true);
            }
        })
    };

    // ======== Events ==========
    let state_apply_callback = {
        let states = states.clone();
        Callback::from(move |value: StateXml| {
            states.update(value.pk, value);
        })
    };

    let predef_apply_callback = {
        let predef_states = predef_states.clone();
        Callback::from(move |(index, value): (usize, StatePredefXml)| {
            let mut predefs = (*predef_states).clone();
            let _ = std::mem::replace(&mut predefs[index], value);
            predef_states.set(predefs);
        })
    };
    
    let on_state_add = {
        let range_type = cell_state.range_type.clone();
        let states = states.clone();
        Callback::from(move |_| {
            let index = states.current().len();
            let name: IString = format!("state-{index}").into();            
            let new_state = match range_type {
                RangeType::DISCRET => {
                    let prev_val = states.current().last()
                        .map(|o| o.value.get_value())
                        .unwrap_or(0);
                    StateXml { 
                        pk: index, 
                        name,
                        value: RangeValue::DiscretConst { value: prev_val },
                        ..Default::default() 
                    }
                },
                RangeType::RANGE => {
                    let prev_val = states.current().last()
                        .map(|o| o.value.get_to())
                        .unwrap_or(0.0);
    
                     StateXml { 
                        pk: index, 
                        name,
                        value: RangeValue::RangeConst { from: prev_val, to: prev_val },
                        ..Default::default() 
                    }
                },            
            };

            states.insert(index, new_state);
        })
    };

    //====== View Items =====
    let data_source_view = {
        let props = yew::props!(data_source::Props {
            ds: cell_state.ds.clone(),
            edit_mode: *edit_mode,
        });
        html! {<DataSourceComponent ..props/>}
    };

    let default_state_view: Html = {
        let default = (*predef_states)[0].clone();
        html! {
            if *edit_mode {
                <StatePredefEditComponent<StatePredefXml> value={default} index={0} apply={predef_apply_callback.clone()}/>
            } else {
                <StatePredefComponent<StatePredefXml> value={default}/>
            }
        }
    };    

    let bad_state_view: Html = {
        let bad = (*predef_states)[1].clone();
        html! {
            if *edit_mode {
                <StatePredefEditComponent<StatePredefXml> value={bad} index={1} apply={predef_apply_callback.clone()}/>
            } else {
                <StatePredefComponent<StatePredefXml> value={bad}/>
            }
        }
    };       

    let states_view = {
        let edit_mode = edit_mode.clone();
        let selected = selected_state.clone();
        states.current().iter()
            .map(move |item| {
                if edit_mode {
                    let props = yew::props!(state::MultystateStateEditProps {
                            value: (*item).clone(),
                            selected: if let Some(selected) = (*selected).clone() {
                                log::debug!("states: {selected:?}");

                                selected.get_index() == item.get_index()
                            } else {
                                false
                            },
                            apply: state_apply_callback.clone(),
                            select: state_select_callback.clone(),
                        });
                    html! { <MultystateStateEditComponent ..props/> }
                } else {
                    html!{ <MultystateStateComponent value={(*item).clone()}/> }
                } 
            })
            .collect::<Vec<_>>()
    };

    html! {
        <fieldset>
            <legend>{"Множественные состояния:"}</legend>

            { data_source_view }
        
            { default_state_view }
            { bad_state_view }     
            
            <div class="flex-box delim-label">
                {"Состояния"}
                if *edit_mode {
                     <button onclick={on_state_add} title="Добавить">{"+"}</button> 
                } 
            </div>
            { states_view }
        </fieldset>
    }
}

