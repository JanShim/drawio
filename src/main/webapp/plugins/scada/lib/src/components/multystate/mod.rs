use std::rc::Rc;
use yew::{function_component, html, use_state, Callback, Html, MouseEvent,};
use yewdux::{use_selector, use_store, Reducer};

use data_source::DataSourceComponent;
use state::MultystateStateComponent;

use crate::{
    errors::CellStateError, model::cell_meta::{
        multystate::{
            self, data_source::DataSourceMeta, state::StateMeta 
        }, CellMeta 
    }, store::cell 
};

pub mod data_source;
pub mod state;

// #[derive(Properties, PartialEq, Debug)]
// pub struct Props {
//     // #[prop_or_default]
//     // pub value: MultystateMeta,
//     #[prop_or_default]
//     pub apply: Callback<MultystateMeta>,
// }

pub struct MultystateAddStateAction;
impl Reducer<cell::CellState> for MultystateAddStateAction {
    fn apply(self, state: Rc<cell::CellState>) -> Rc<cell::CellState> {
        let mut multystate = state.meta.multystate.clone()
            .expect(format!("{}", CellStateError::NotMultystate).as_str());

        multystate.states.push(StateMeta { pk: multystate.states.len(), ..Default::default() });

        cell::CellState {
           cell: state.cell.clone(),
           meta: CellMeta { multystate: Some(multystate), ..state.meta.clone() },
        }
        .into()
    }
}

#[function_component(MultystateComponent)]
pub fn component() -> Html {
    let (_, cell_store_dispatch) = use_store::<cell::CellState>();
    let multy_state = use_selector(|cell_state: &cell::CellState| 
        cell_state.meta.multystate.clone()
            .expect(format!("{}", CellStateError::NotMultystate).as_str())
    );    
    
    /* #region selected_state */
    let selected_state = use_state(|| {
        let value: Option<StateMeta> = None;
        value
    });

    let state_select_callback = {
        let selected = selected_state.clone();
        Callback::from(move |value: Option<StateMeta>| {
            // log::debug!("state_select_callback: -> {meta:?}");

            // change selected
            selected.set(value);

        })
    };

    // let state_apply_callback = {
    //     let multy_state = multy_state.clone();
    //     Callback::from(move |value: StateMeta| {
    //         multy_state.dispatch(MultystateMetaAction::ApplyMultystateStateMeta(value));
    //     })
    // };


    /* #endregion */


    // -------------------------------------------------------
    // let on_state_add = {
    //     // let multy_state = multy_state.clone();
    //     // Callback::from(move |_| multy_state.dispatch(MultystateMetaAction::CreateState))
    //     cell_store_dispatch.apply_callback(|_| cell::MultystateAddStateAction)
    // };

    let on_state_add = cell_store_dispatch.apply_callback(|_| MultystateAddStateAction); 

    // // apply changes to multystate meta
    // let state_apply_callback: Callback<Rc<StateMeta>> = cell_store_dispatch
    //     .reduce_mut_callback_with(|state, meta: Rc<StateMeta>| {
    //         // log::debug!("state_apply_callback -> {state:?} || {meta:?}");
    //         // state.set_multystate_state_style(meta.get_index(), meta.style.clone()).ok();
    //     });

    // let data_soure_apply = {
    //     let multy_state = multy_state.clone();
    //     Callback::from(move |ds: DataSourceMeta| {
    //         multy_state.dispatch(MultystateMetaAction::ApplyDataSource(ds));
    //     })
    // };
    // let data_soure_apply = cell_store_dispatch.apply_callback(|ds: DataSourceMeta| MultystateApplyDsAction(ds)); 


    // let apply_multystate = {
    //         let multy_state = multy_state.clone();
    //         let value_apply = value_apply.clone();
    //         Callback::from(move |_: MouseEvent| {
    //             value_apply.emit((*multy_state).clone());
    //         })
    //     };

    // ------------ View Items
    let data_source_view = {
        // let data_soure_apply = data_soure_apply.clone();
        let props = yew::props!(data_source::Props {
            ds: multy_state.data_source.clone(),
            // apply: data_soure_apply,
        });
        html! {<DataSourceComponent ..props/>}
    };

    let states_view = {
        let selected = selected_state.clone();
//        if let Some(multy) = multy_state.clone() {
        multy_state.states.iter().enumerate()
            .map(|(id, meta)| {
                let props = state::Props {
                    value: meta.clone(),
                    selected: if let Some(selected) = (*selected).clone() {
                        selected.get_index() == id
                    } else {
                        false
                    },
                    select: state_select_callback.clone(),
                    // apply: state_apply_callback.clone(),
                };
                html! { <MultystateStateComponent ..props/> }
            })
            .collect::<Html>()
  //      }
    };

    html! {
        <>
        <pre>{
            format!("{:?}", *multy_state)
        }</pre>
        // <div class="flex-box-2" style="background-color: yellow;">
        //     <button onclick={apply_multystate}><img src="images/checkmark.gif"/></button>
        // </div>            
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