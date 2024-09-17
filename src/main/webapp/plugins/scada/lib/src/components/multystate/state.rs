use std::rc::Rc;
use yew::{html, function_component, use_effect_with, use_reducer, Callback, Html, MouseEvent, Properties};
use yewdux::{use_store, Reducer};

use crate::{
    errors::CellStateError, 
    model::cell_meta::{
        CellMeta,
        multystate::state::{StateAction, StateMeta}, 
    }, 
    store::cell,
};

pub struct MultystateApplyStateAction(StateMeta);
impl Reducer<cell::CellState> for MultystateApplyStateAction {
    fn apply(self, state: Rc<cell::CellState>) -> Rc<cell::CellState> {
        let mut multystate = state.meta.multystate.clone()
            .expect(format!("{}", CellStateError::NotMultystate).as_str());

        let new_state = self.0;            
        let index = new_state.get_index();
        let states = &mut multystate.states;
        states[index] = StateMeta { ..new_state };

        log::debug!("states: {states:?}");

        cell::CellState {
            cell: state.cell.clone(),
            meta: CellMeta { 
                    multystate: Some(multystate), 
                    ..state.meta.clone() 
                },
            }
            .into()            
    }
}


#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub selected: bool,
    pub value: StateMeta,
    pub select: Callback<Option<StateMeta>>,
}

#[function_component(MultystateStateComponent)]
pub fn component(Props {
    value, 
    select, 
    selected
}: &Props) -> Html {
    // cell meta storage
    let (cell_state, cell_state_dispatch) = use_store::<cell::CellState>();

    let my_state = use_reducer(|| value.clone());
    {
        let my_state = my_state.clone();
        use_effect_with(value.clone(), move |value| {
            my_state.dispatch(StateAction::Clone((*value).clone()));
        });
    }

    let toggle_edit = {
        let my_state = my_state.clone();
        let select = select.clone();
        Callback::from(move |_: MouseEvent| { select.emit(Some((*my_state).clone())) })
    };      

    let toggle_apply = {
        // let cell_state = cell_state.clone();
        let my_state = my_state.clone();
        let select = select.clone();
        Callback::from(move |_: MouseEvent| { 
            if let Some(style) = cell_state.get_cell_style().ok() {
                let state = StateMeta { style, ..(*my_state).clone() };
                cell_state_dispatch.apply(MultystateApplyStateAction(state));
            }

            select.emit(None);  // remove selection
        })
    };   
    // {   // effect он toggle_apply
    //     let my_state = my_state.clone();
    //     let apply = apply.clone();
    //     use_effect_with((*my_state).clone(), move |v| {
    //         apply.emit((*v).clone());
    //     })
    // }

    // --- view items
    let view_mode = html! {
        <td>{ my_state.name.as_str() } {my_state.style.as_str() }</td>
    };

    let edit_mode = html! {
        <td>
            <table width="100%">
                <tr>
                    <td>{ my_state.name.as_str() }</td>
                </tr>
                <tr>
                    <td>{"here must be style"}</td>
                </tr>
            </table>
        </td>
    };


    let img = {
        if *selected { 
           html! { <img src="images/checkmark.gif" onclick={toggle_apply}/> }
        }
         else {
           html! { <img src="images/edit16.png" onclick={toggle_edit}/> }
        }
    };

    // item view
    html! {
        <table class="prop-table">
        <td class="label" width="10">{my_state.pk.clone()}</td>
        <td>{ 
            if *selected {
                { edit_mode }
            } else {
                { view_mode }
            }
         }</td>
        <td class="img">{ img }</td>
        </table>
    }
    
}