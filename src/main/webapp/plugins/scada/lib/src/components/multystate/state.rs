use yew::{function_component, html, use_state, Callback, Html, MouseEvent, Properties};
use yewdux::{use_selector, use_store};

use crate::{
    model::cell_meta::multystate::state::StateMeta, 
    store::cell
};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub index: usize,
    pub select: Callback<Option<usize>>,
    pub selected: bool,
}

#[function_component(MultystateStateComponent)]
pub fn component(Props { index, select, selected }: &Props) -> Html {
    let (state, _) = use_store::<cell::State>();

    let multystate_state = use_state(|| {
        match state.get_multystate_state(*index).ok() {
            Some(state) => (*state).clone(),
            _ => StateMeta::default()
        }
    });

    let togle_edit = {
        let index = *index;
        let select = select.clone();
        Callback::from(move |_: MouseEvent| select.emit(Some(index)))
    };      


    let togle_apply = {
        let state = state.clone();
        let index = *index;
        let select = select.clone();
        Callback::from(move |_: MouseEvent| { 
            log::debug!("togle_apply {index:#?}");

            let style = state.get_cell_style().unwrap();
            log::debug!("togle_apply {style:#?}");


            select.emit(None); 
        })
    };   


    let view_mode = html! {
        <td>{ (*multystate_state).name.as_str() }</td>
    };

    let edit_mode = html! {
        <td>
            <table width="100%">
                <tr>
                    <td>{ (*multystate_state).name.as_str() }</td>
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
           html! { <img src="images/checkmark.gif" onclick={togle_apply}/> }
        } else {
           html! { <img src="images/edit16.png" onclick={togle_edit}/> }
        }
    };

    // item view
    html! {
        <table class="prop-table">
        <td class="label" width="10">{index}</td>
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