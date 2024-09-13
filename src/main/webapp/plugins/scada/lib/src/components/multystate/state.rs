use std::rc::Rc;

use yew::{function_component, html, use_effect, use_effect_with, use_reducer, use_state, use_state_eq, Callback, Html, MouseEvent, Properties};
use yewdux::{dispatch, use_selector, use_store};

use crate::{
    model::cell_meta::multystate::state::{StateMeta, StateAction}, 
    store::cell
};


#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub select_callback: Callback<Option<Rc<StateMeta>>>,
    pub apply_callback: Callback<Rc<StateMeta>>,
    pub selected: bool,
    pub meta: Rc<StateMeta>,
}

#[function_component(MultystateStateComponent)]
pub fn component(Props {
    meta, 
    select_callback, 
    apply_callback, 
    selected
}: &Props) -> Html {

    // cell meta storage
    let (cell_state, cell_state_dispatch) = use_store::<cell::State>();

    let my_state = {
            let meta =  meta.clone();
            use_reducer(move || (*meta).clone())
        };
    {
        let my_state = my_state.clone();
        use_effect_with((*meta).clone(), move |meta| {
            my_state.dispatch(StateAction::Clone((*meta).clone()));
        });
    }


    let toggle_edit = {
        let my_state = my_state.clone();
        let select_callback = select_callback.clone();
        Callback::from(move |_: MouseEvent| {
            select_callback.emit(Some(Rc::new((*my_state).clone())))
        })
    };      

    let is_changed = use_state_eq(|| false);
    let toggle_apply = {
        let cell_state = cell_state.clone();
        let my_state = my_state.clone();
        let is_changed = is_changed.clone();
        let select_callback = select_callback.clone();
        // let cell_state_dispatch.

        Callback::from(move |_: MouseEvent| { 
            if let Some(style) = cell_state.get_cell_style().ok() {
                my_state.dispatch(StateAction::SetStyle(style));    // dispatch set style
                is_changed.set(true); // mark as changed
                select_callback.emit(None);  // remove selection
            }
        })
    };   
    // effect он toggle_apply
    {   
        let is_changed = is_changed.clone(); 
        let my_state = my_state.clone();
        let apply_callback = apply_callback.clone();
        use_effect(move || {
            if *is_changed {
                apply_callback.emit(Rc::new((*my_state).clone()));
            }
        });
    }

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
                    <td>{"aaaaaaaaaaaaa"}</td>
                </tr>
            </table>
        </td>
    };


    // let state = match state.get_multystate_state(*index).ok() {
    //     Some(state) => {
    //         html! {<div>{ format!("{state:#?}") }</div>}
    //     },
    //     _ => html! {<div>{"state not found"}</div>}
    // }; 

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