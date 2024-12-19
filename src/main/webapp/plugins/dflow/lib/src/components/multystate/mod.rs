use yew::prelude::*;
use yew_hooks::{use_list, use_unmount};
use yewdux::use_selector;
use common_model::{
    data_source::DataSourceXml, dflow_cell::DFlowVariant,
    multystate::{range::RangeType, state::StateXml, state_predef::StatePredefXml, MultystateXml}
};

use state_predef::{StatePredefComponent, StatePredefEditComponent};
use state::{MultystateStateComponent, MultystateStateEditComponent};
use states::StatesSelector;

use crate::{
    components::{data_source::{self, DataSource}, shared::use_list_selected},
    store::cell
};

// pub mod type_selector;
pub mod states;
pub mod state;
pub mod state_rect;
pub mod state_predef;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub edit_mode: bool,
    pub value: Option<MultystateXml>,
    // pub on_detals_apply: Callback<DFlowVariant>,    // callback for applyed notification
}

#[function_component]
pub fn MultystateComponent(Props {
    edit_mode ,
    value,
    // on_detals_apply,
}: &Props) -> Html
{
    use_unmount(|| {
        log::debug!("MultystateComponent unmount");
    });

    // let meta = use_state(|| value.clone());
    // let range_type = use_state(|| meta.range_type.clone());
    // let data_source = use_state(|| meta.ds.clone());
    // let predef_states = use_state(|| meta.predef.clone());
    // let states = use_list(meta.states.clone());
    // {
    //     let range_type = range_type.clone();
    //     let data_source = data_source.clone();
    //     let predef_states = predef_states.clone();
    //     let states = states.clone();
    //     use_effect_with(value.clone(), move |m| {
    //         range_type.set(m.range_type.clone());
    //         data_source.set(m.ds.clone());
    //         predef_states.set(m.predef.clone());
    //         states.set(m.states.clone());
    //     })
    // }

    // let (selected, select_callback) = use_list_selected::<StateXml>();

    // // start apply process if true
    // let start_apply = use_selector(|state: &cell::State | state.start_apply);
    // {
    //     let on_detals_apply = on_detals_apply.clone();
    //     let data_source = data_source.clone();
    //     let predef_states = predef_states.clone();
    //     let states = states.clone();
    //     let range_type = range_type.clone();
    //     use_effect_with(*start_apply, move |start| {
    //         if *start {
    //             let new_state = MultystateXml {
    //                 range_type: (*range_type).clone(),
    //                 ds: (*data_source).clone(),
    //                 predef: (*predef_states).clone(),
    //                 states: states.current().clone(),
    //             };

    //             let new_variant = DFlowVariant::Multystate(new_state);
    //             log::debug!("NEW MULTY {:?}", new_variant);
    //             on_detals_apply.emit(new_variant);
    //         }
    //     })
    // };

    // // ======== Events ==========
    // let state_apply_callback = {
    //     let states = states.clone();
    //     let range_type = range_type.clone();
    //     Callback::from(move |value: StateXml| {
    //         match *range_type {
    //             RangeType::DISCRET => states.update(value.pk, value),
    //             RangeType::RANGE => {
    //                 let len = states.current().len();
    //                 let index = len - value.pk - 1;     // for range invers index
    //                 states.update(index, value)
    //             },
    //         };
    //     })
    // };

    // let predef_apply_callback = {
    //     let predef_states = predef_states.clone();
    //     Callback::from(move |(index, value): (usize, StatePredefXml)| {
    //         let mut predefs = (*predef_states).clone();
    //         let _ = std::mem::replace(&mut predefs[index], value);
    //         predef_states.set(predefs);
    //     })
    // };

    // let apply_ds = {
    //         let data_source = data_source.clone();
    //         Callback::from(move |ds: DataSourceXml| {
    //             data_source.set(ds);
    //         })
    //     };

    // let on_range_type_change = {
    //         let range_type_handler = range_type.clone();
    //         let states = states.clone();
    //         Callback::from(move |range_type: RangeType| {
    //             states.clear();
    //             // store_state_dispatch.apply(SetRangeTypeAction(range_type));
    //             range_type_handler.set(range_type)
    //         })
    //     };

    // //====== View Items =====
    // let data_source_view = {
    //         let data_source = data_source.clone();
    //         let apply_ds = apply_ds.clone();
    //         let props = yew::props!(data_source::Props {
    //             ds: (*data_source).clone(),
    //             edit_mode: *edit_mode,
    //             on_apply: apply_ds,
    //         });
    //         html! {<DataSource ..props/>}
    //     };

    // let default_state_view: Html = {
    //         let default = (*predef_states)[0].clone();
    //         html! {
    //             if *edit_mode {
    //                 <StatePredefEditComponent<StatePredefXml> value={default} index={0} apply={predef_apply_callback.clone()}/>
    //             } else {
    //                 <StatePredefComponent<StatePredefXml> value={default}/>
    //             }
    //         }
    //     };

    // let bad_state_view: Html = {
    //         let bad = (*predef_states)[1].clone();
    //         html! {
    //             if *edit_mode {
    //                 <StatePredefEditComponent<StatePredefXml> value={bad} index={1} apply={predef_apply_callback.clone()}/>
    //             } else {
    //                 <StatePredefComponent<StatePredefXml> value={bad}/>
    //             }
    //         }
    //     };

    // let states_view = {
    //         let range_type = range_type.clone();
    //         let edit_mode = edit_mode.clone();
    //         let selected = selected.clone();
    //         states.current().iter()
    //             .map(move |item| {
    //                 if edit_mode {
    //                     let props = yew::props!(state::MultystateStateEditProps {
    //                             value: (*item).clone(),
    //                             selected: if let Some(selected) = (*selected).clone() {
    //                                 selected.get_index() == item.get_index()
    //                             } else {
    //                                 false
    //                             },
    //                             apply: state_apply_callback.clone(),
    //                             select: select_callback.clone(),
    //                         });
    //                     html! { <MultystateStateEditComponent ..props/> }
    //                 } else {
    //                     html!{ <MultystateStateComponent value={(*item).clone()} range_type={(*range_type).clone()}/> }
    //                 }
    //             })
    //             .collect::<Vec<_>>()
    //     };

    html! {
        <fieldset>
            <legend>{"Множественные состояния:"}</legend>
            // { data_source_view }

            // { default_state_view }
            // { bad_state_view }

            // <StatesSelector
            //     edit_mode={edit_mode}
            //     states={states.clone()}
            //     range_type={ (*range_type).clone() }
            //     {on_range_type_change}
            // />

            // { states_view }

            <pre>{ format!("{value:?}") }</pre>
        </fieldset>
    }
}
