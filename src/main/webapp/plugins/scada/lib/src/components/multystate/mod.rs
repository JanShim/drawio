use common_model::multystate::{state::StateXml, MultystateXml};
use state_edit::MultystateStateEditComponent;
use yew::{function_component, html, use_state, Callback, Html, Properties,};
use yewdux::{use_selector, use_store, };

use data_source::DataSourceComponent;
use state::MultystateStateComponent;

use crate::{
    errors::CellStateError, model::cell_meta::{multystate::MultystateAddStateAction, CellMetaVariant}, store::cell
};

pub mod data_source;
pub mod state;
pub mod state_edit;
pub mod state_rect;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub edit_mode: bool,
}

#[function_component(MultystateComponent)]
pub fn component(Props { edit_mode }: &Props) -> Html {
    let (_, cell_store_dispatch) = use_store::<cell::State>();

    let multy_state = use_selector(|cell_state: &cell::State| {
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
            ds: multy_state.data_source.clone(),
            edit_mode: *edit_mode,
        });
        html! {<DataSourceComponent ..props/>}
    };

    let states_view = {
        let selected = selected_state.clone();
        multy_state.states.iter().enumerate()
            .map(|(id, meta)| {
                if *edit_mode {
                    let props = state_edit::Props {
                        value: meta.clone(),
                        selected: if let Some(selected) = (*selected).clone() {
                            selected.get_index() == id
                        } else {
                            false
                        },
                        select: state_select_callback.clone(),
                    };
                    html! { <MultystateStateEditComponent ..props/> }
                } else {
                    let props = state::Props {
                        value: meta.clone(),
                    };

                    html!{ <MultystateStateComponent ..props/> }
                }   
            })
            .collect::<Html>()
    };

    html! {
        <>
        <hr/>
        { data_source_view }
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

