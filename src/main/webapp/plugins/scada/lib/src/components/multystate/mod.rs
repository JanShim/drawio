use common_model::multystate::{state::StateXml, state_predef::StatePredefXml, MultystateXml};
use state_predef::{StatePredefComponent, StatePredefEditComponent};
use yew::{function_component, html, use_state, Callback, Html, Properties,};
use yewdux::{use_selector, use_store, };

use data_source::DataSourceComponent;
use state::{MultystateStateComponent, MultystateStateEditComponent};

use crate::{
    errors::CellStateError, model::cell_meta::CellMetaVariant, store::cell::{self, MultystateAddStateAction}
};

pub mod data_source;
pub mod state;
pub mod state_rect;
pub mod state_predef;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub edit_mode: bool,
}

#[function_component]
pub fn MultystateComponent(Props { edit_mode }: &Props) -> Html {
    let (_, cell_store_dispatch) = use_store::<cell::State>();

    let cell_state = use_selector(|cell_state: &cell::State| {
        if let CellMetaVariant::Multystate(multystate) = cell_state.meta.data.clone() {
			return multystate;
		};
        log::error!("{}", CellStateError::NotMultystate);
        MultystateXml::default().into()
    });    
    
    /* #region selected_state */
    let selected_state = use_state(|| {
        let value: Option<StateXml> = None;
        value
    });

    let state_select_callback = {
        let selected = selected_state.clone();
        Callback::from(move |value: Option<StateXml>| {
            // change selected
            selected.set(value);

        })
    };
    /* #endregion */

    // -------------------------------------------------------
    let on_state_add = cell_store_dispatch.apply_callback(|_| MultystateAddStateAction); 

    //====== View Items =====
    let data_source_view = {
        let props = yew::props!(data_source::Props {
            ds: cell_state.ds.clone(),
            edit_mode: *edit_mode,
        });
        html! {<DataSourceComponent ..props/>}
    };

    let default_state_view: Html = {
        let default = cell_state.predef[0].clone();
        html! {
            if *edit_mode {
                <StatePredefEditComponent<StatePredefXml> value={default}/>
            } else {
                <StatePredefComponent<StatePredefXml> value={default}/>
            }
        }
    };    

    let bad_state_view: Html = {
        let bad = cell_state.predef[1].clone();
        html! {
            if *edit_mode {
                <StatePredefEditComponent<StatePredefXml> value={bad}/>
            } else {
                <StatePredefComponent<StatePredefXml> value={bad}/>
            }
        }
    };       

    let states_view = {
        let selected = selected_state.clone();
        cell_state.states.iter().enumerate()
            .map(|(id, meta)| {
                if *edit_mode {
                    let props = yew::props!(state::MultystateStateEditProps {
                        value: meta.clone(),
                        selected: if let Some(selected) = (*selected).clone() {
                            selected.get_index() == id
                        } else {
                            false
                        },
                        select: state_select_callback.clone(),
                    });
                    html! { <MultystateStateEditComponent ..props/> }
                } else {
                    html!{ <MultystateStateComponent value={meta.clone()}/> }
                }   
            })
            .collect::<Html>()
    };

    html! {
        <>
        <hr/>
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
        </>
    }
}

