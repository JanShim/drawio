use std::{cell::RefCell, rc::Rc};

use common_model::{data_source::DataSourceXml, multystate::{range::RangeType, state::StateXml, state_predef::StatePredefXml, MultystateXml}};
use state_predef::{StatePredefComponent, StatePredefEditComponent};
use states::StatesSelector;
use yew::{function_component, html, use_effect_with, use_state, Callback, Html, Properties, UseStateHandle};
use yew_hooks::{use_list, use_unmount};
use yewdux::use_selector;

use data_source::DataSourceComponent;
use state::{MultystateStateComponent, MultystateStateEditComponent};

use crate::{
    errors::CellStateError, model::cell_meta::{CellMeta, CellMetaVariant, CellType}, store::cell
};

pub mod data_source;
// pub mod type_selector;
pub mod states;
pub mod state;
pub mod state_rect;
pub mod state_predef;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub edit_mode: bool,
    pub cell_meta: Rc<RefCell<CellMeta>>,
    pub on_detals_apply: Callback<CellMetaVariant>,    // callback for applyed notification
}

#[function_component]
pub fn MultystateComponent(Props { 
    edit_mode , 
    on_detals_apply, 
    cell_meta,
}: &Props) -> Html 
{
    use_unmount(|| {
        log::debug!("MultystateComponent unmount");
    });

    let multy_meta = use_state(|| {
            if let Ok(multystate) = cell_meta.borrow().get_multystate_meta() {
                return multystate;
            };
            log::warn!("{}", CellStateError::NotMultystate);
            MultystateXml::default()        
        });
    let range_type = use_state(|| multy_meta.range_type.clone());
    let data_source = use_state(|| multy_meta.ds.clone());
    let predef_states = use_state(|| multy_meta.predef.clone());
    let states = use_list(multy_meta.states.clone());

    /* #region selected_state */
    let selected_state = use_state(|| {
        let value: Option<StateXml> = None;
        value
    });

    let state_select_callback = {
        let selected = selected_state.clone();
        Callback::from(move |value: Option<StateXml>| {
            // log::debug!("state_select_callback: {value:?}");
            selected.set(value);  // change selected
        })
    };
    /* #endregion */

    // start apply process if true
    let start_apply = use_selector(|state: &cell::State | state.start_apply);
    {    
        let on_detals_apply = on_detals_apply.clone();
        let data_source = data_source.clone();
        let predef_states = predef_states.clone();
        let states = states.clone();
        let range_type = range_type.clone();
        use_effect_with(*start_apply, move |start| {
            if *start {
                let new_state = MultystateXml {
                    range_type: (*range_type).clone(),
                    ds: (*data_source).clone(),
                    predef: (*predef_states).clone(),
                    states: states.current().clone(),
                };

                let new_variant = CellMetaVariant::Multystate(new_state);

                log::debug!("NEW MULTY {:?}", new_variant);      

                on_detals_apply.emit(new_variant);
            }
        })
    };

    // ======== Events ==========
    let state_apply_callback = {
        let states = states.clone();
        let range_type = range_type.clone();
        Callback::from(move |value: StateXml| {
            match *range_type {
                RangeType::DISCRET => states.update(value.pk, value),
                RangeType::RANGE => {
                    let len = states.current().len();
                    let index = len - value.pk - 1;     // for range invers index
                    states.update(index, value)
                },
            };
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

    let apply_ds = {
            let data_source = data_source.clone();
            Callback::from(move |ds: DataSourceXml| {
                data_source.set(ds);
            })
        };

    let on_range_type_change = {
            let range_type_handler = range_type.clone();
            let states = states.clone();
            Callback::from(move |range_type: RangeType| {
                states.clear();
                // store_state_dispatch.apply(SetRangeTypeAction(range_type));
                range_type_handler.set(range_type)
            })
        };

    //====== View Items =====
    let data_source_view = {
            let data_source = data_source.clone();
            let apply_ds = apply_ds.clone();
            let props = yew::props!(data_source::Props {
                ds: (*data_source).clone(),
                edit_mode: *edit_mode,
                apply: apply_ds,
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
            let range_type = range_type.clone();
            let edit_mode = edit_mode.clone();
            let selected = selected_state.clone();
            states.current().iter()
                .map(move |item| {
                    if edit_mode {
                        let props = yew::props!(state::MultystateStateEditProps {
                                value: (*item).clone(),
                                selected: if let Some(selected) = (*selected).clone() {
                                    selected.get_index() == item.get_index()
                                } else {
                                    false
                                },
                                apply: state_apply_callback.clone(),
                                select: state_select_callback.clone(),
                            });
                        html! { <MultystateStateEditComponent ..props/> }
                    } else {
                        html!{ <MultystateStateComponent value={(*item).clone()} range_type={(*range_type).clone()}/> }
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
            
            <StatesSelector 
                edit_mode={edit_mode} 
                states={states.clone()} 
                range_type={ (*range_type).clone() }
                {on_range_type_change}
            />

            { states_view }
        </fieldset>
    }
}

