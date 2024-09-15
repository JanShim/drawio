use std::rc::Rc;
use yew::{function_component, html, use_reducer, use_state, Callback, Html, MouseEvent,};
use yewdux::use_store;

use data_source::DataSourceComponent;
use state::MultystateStateComponent;

use crate::{
    model::cell_meta::multystate::{data_source::DataSource, state::StateMeta, MultystateMetaAction},
    store::cell,
};

pub mod data_source;
pub mod state;

#[function_component(MultystateComponent)]
pub fn component() -> Html {
    let (cell_store, cell_store_dispatch) = use_store::<cell::CellState>();

    /* #region selected_state */
    let selected_state = use_state(|| {
        let value: Option<Rc<StateMeta>> = None;
        value
    });

    let state_select_callback = {
        let selected = selected_state.clone();
        Callback::from(move |meta: Option<Rc<StateMeta>>| {
            // log::debug!("state_select_callback: -> {meta:?}");

            // change selected
            selected.set(meta);
        })
    };
    /* #endregion */

    let multy_state = use_reducer(|| cell_store.meta.clone().multystate.unwrap_or_default());

    // -------------------------------------------------------
    let cell_meta_apply: Callback<MouseEvent> = {
            let multy_state = multy_state.clone();
            cell_store_dispatch.reduce_mut_callback(move |cell_state| {
                log::debug!("cell_meta_apply:: multy {:?}", *multy_state);

                cell_state.set_multystate((*multy_state).clone());
                cell_state.apply_meta_to_cell();
            }
        )};

    let on_state_add = {
        let multy_state = multy_state.clone();
        Callback::from(move |_| multy_state.dispatch(MultystateMetaAction::CreateState))
    };

    // // apply changes to multystate meta
    // let state_apply_callback: Callback<Rc<StateMeta>> = cell_store_dispatch
    //     .reduce_mut_callback_with(|state, meta: Rc<StateMeta>| {
    //         // log::debug!("state_apply_callback -> {state:?} || {meta:?}");
    //         // state.set_multystate_state_style(meta.get_index(), meta.style.clone()).ok();
    //     });

    let data_soure_apply = {
        let multy_state = multy_state.clone();
        Callback::from(move |ds: DataSource| {
            multy_state.dispatch(MultystateMetaAction::ApplyDataSource(ds));
        })
    };

    // ------------ View Items
    let data_source_view = {
        let data_soure_apply = data_soure_apply.clone();
        let props = yew::props!(data_source::Props {
            ds: multy_state.data_source.clone(),
            apply: data_soure_apply,
        });
        html! {<DataSourceComponent ..props/>}
    };

    let states_view = {
        let selected = selected_state.clone();
        multy_state.states.iter().enumerate()
            .map(|(id, meta)| {
                let props = state::Props {
                    selected: if let Some(selected) = (*selected).clone() {
                        selected.get_index() == id
                    } else {
                        false
                    },
                    select_callback: state_select_callback.clone(),
                    // apply_callback: state_apply_callback.clone(),
                    meta: Rc::new(meta.clone()),
                };
                html! { <MultystateStateComponent ..props/> }
            })
            .collect::<Html>()

        // if let Some(ms) = cell_store.get_multystate().ok() {
        //     ms.states
        //         .iter()
        //         .enumerate()
        //         .map(|(id, meta)| {
        //             let props = state::Props {
        //                 selected: if let Some(selected) = (*selected).clone() {
        //                     selected.get_index() == id
        //                 } else {
        //                     false
        //                 },
        //                 select_callback: state_select_callback.clone(),
        //                 apply_callback: state_apply_callback.clone(),
        //                 meta: Rc::new(meta.clone()),
        //             };
        //             html! { <MultystateStateComponent ..props/> }
        //         })
        //         .collect::<Html>()
        // } else {
        //     empty
        // }
    };

    html! {
        <>
        <pre>{
            format!("{:?}", *multy_state)
        }</pre>
        <div class="flex-box-2">
           <button onclick={cell_meta_apply}><img src="images/checkmark.gif"/></button>
        </div>
        <hr/>
        { data_source_view }
        <div class="flex-box">{"Состояния"}<button onclick={on_state_add}>{"+"}</button></div>
        { states_view }
        </>
    }
}


/*

/// reducer's Action
enum CounterAction {
    Double,
    Square,
}

/// reducer's State
struct CounterState {
    counter: i32,
}

impl Default for CounterState {
    fn default() -> Self {
        Self { counter: 1 }
    }
}

impl Reducible for CounterState {
    /// Reducer Action Type
    type Action = CounterAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_ctr = match action {
            CounterAction::Double => self.counter * 2,
            CounterAction::Square => self.counter.pow(2),
        };

        Self { counter: next_ctr }.into()
    }
}
*/