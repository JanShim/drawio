use yew::{function_component, html, use_effect_with, use_reducer, use_state, use_state_eq, Callback, Html, MouseEvent, Properties};
use yewdux::{dispatch, use_selector, use_store};

use crate::{
    model::cell_meta::multystate::state::{StateMeta, StateAction}, 
    store::cell
};


#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub select: Callback<Option<usize>>,
    pub selected: bool,
    pub meta: StateMeta,
}

#[function_component(MultystateStateComponent)]
pub fn component(Props {meta, select, selected}: &Props) -> Html {
    let (cell_state, cell_state_dispach) = use_store::<cell::State>();

    // let my_state = use_state(|| {
    //     log::debug!("call state for: {index}");

    //     match state.get_multystate_state(*index).ok() {
    //         // Some(my_state) => {
    //         //     StateMeta {pk: (*index).to_string(), ..(*my_state).clone()}
    //         // },
    //         Some(state) => (*state).clone(),          
    //         _ => StateMeta::default()
    //     }
    // });
    // let meta_c = (*meta).clone();
    let my_state = use_reducer(|| StateMeta::default());
    {
        let my_state = my_state.clone();
        use_effect_with((*meta).clone(), move |meta| {
            my_state.dispatch(StateAction::Clone((*meta).clone()));
        });
    }

    let toggle_edit = {
        let my_state = my_state.clone();
        let select = select.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(index) = my_state.pk.parse::<usize>().ok() {
                select.emit(Some(index))
            }
        })
    };      


    let toggle_apply = {
        let cell_state = cell_state.clone();
        let my_state = my_state.clone();
        let select = select.clone();
        Callback::from(move |_: MouseEvent| { 
            if let Some(style) = cell_state.get_cell_style().ok() {
                // log::debug!("style {style:#?}");
                // // let StateMeta {pk, name, style, selected} = (*my_state).clone();
                // log::debug!("my_state {my_state:#?}");
    
                // let mut new_state = (*my_state).clone();
                // new_state.style = style;
                // log::debug!("new_state: {new_state:#?}");

                my_state.dispatch(StateAction::SetStyle(style));

                // let dispach = cell_state_dispach.clone();
                // let _ = my_state.get_index()
                //     .map(move |i| {
                //         dispach.reduce_mut(move |s| {
                //             s.set_multystate_state_style(i, style).ok().unwrap();
                //         })
                //     });
    
                // log::debug!("my_state!!!!!: {my_state:#?}");
                
                // remove selection
                select.emit(None); 
            }
        })
    };   


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
        } else {
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