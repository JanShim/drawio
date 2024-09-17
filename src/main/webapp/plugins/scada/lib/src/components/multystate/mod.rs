use yew::{function_component, html, use_state, Callback, Html, Properties,};
use yewdux::{use_selector, use_store, };

use data_source::DataSourceComponent;
use state::MultystateStateComponent;

use crate::{
    errors::CellStateError, model::cell_meta::{
        multystate::{state::StateMeta, MultystateAddStateAction},
        CellMeta 
    }, store::cell 
};

pub mod data_source;
pub mod state;

// struct ApplyMultyStateMeta(MultystateMeta);
// impl Reducer<cell::CellState> for ApplyMultyStateMeta {
//     fn apply(self, state: Rc<cell::CellState>) -> Rc<cell::CellState> {
//         cell::CellState {
//             meta: CellMeta { multystate: Some(self.0), ..(state.meta.clone()) },
//             cell: state.cell.clone(),
//         }.into()        
//     }
// }

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub edit_mode: bool,
}

#[function_component(MultystateComponent)]
pub fn component(Props { edit_mode }: &Props) -> Html {
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
            // change selected
            selected.set(value);

        })
    };
    /* #endregion */


    // -------------------------------------------------------
    let on_state_add = cell_store_dispatch.apply_callback(|_| MultystateAddStateAction); 

    // ------------ View Items
    let data_source_view = {
        let props = yew::props!(data_source::Props {
            ds: multy_state.data_source.clone(),
            edit_mode: *edit_mode,
        });
        html! {<DataSourceComponent ..props/>}
    };

    let states_view = {
        let selected = selected_state.clone();
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
                };
                html! { <MultystateStateComponent ..props/> }
            })
            .collect::<Html>()
    };

    html! {
        <>
        // <pre>{ format!("{:?}", *multy_state) }</pre>
        <hr/>
        { data_source_view }
        <div class="flex-box">{"Состояния"}
            if *edit_mode {
                <button onclick={on_state_add}>{"+"}</button>
            } 
        </div>
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