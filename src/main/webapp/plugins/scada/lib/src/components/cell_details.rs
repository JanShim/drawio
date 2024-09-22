use yew::prelude::*;
use yewdux::{use_selector, use_store};

use crate::{components::{
        multystate::{self, MultystateComponent}, undefiend::{self, UndefiendComponent}, value::{self, ValueComponent}, widget::{self, WidgetComponent}
    }, 
    model::cell_meta::{value::{ApplyValueMetaAction, ValueMeta}, CellMetaVariant, CellType}, 
    store::cell::{self, SetCellTypeAction}
};


#[function_component(CellDetailsComponent)]
pub fn component() -> Html {
    let (cell_state, cell_state_dispatch) = use_store::<cell::CellState>();
    let cell_meta = use_selector(|cell_state: &cell::CellState| cell_state.meta.clone());

    let edit_mode = use_state(|| false);

    let value_apply = cell_state_dispatch.apply_callback(|value: ValueMeta| ApplyValueMetaAction(value));

    let edit_mode_toggle = {
            let edit_mode = edit_mode.clone();
            Callback::from(move |_: MouseEvent| { edit_mode.set(true); })
        };

    let cell_details_apply: Callback<MouseEvent> = {
        let edit_mode = edit_mode.clone();
        let cell_meta = cell_meta.clone();
        Callback::from(move |_: MouseEvent| {
            let meta = cell_state.cell.set_meta(&cell_meta).ok();
            log::debug!("NEW CELL META:: {:?}", meta);
            edit_mode.set(false);
        })
    };

    let cell_type_apply = {
        let cell_state_dispatch = cell_state_dispatch.clone();
        Callback::from(move |cell_type: CellType| {
            cell_state_dispatch.apply(SetCellTypeAction(cell_type));
        })
    };

    // component views
    let header = html!{
        <div class="flex-box-2 delim-label" >
        if *edit_mode {
            <button onclick={cell_details_apply}><img src="images/checkmark.gif" width="16" height="16"/></button>
        } else {
            <button onclick={edit_mode_toggle}><img src="images/edit16.png"/></button>
        }
        </div>           
    };

    let details_vew = {
        let header = header.clone();
        let edit_mode = edit_mode.clone();
        match &cell_meta.data {
            CellMetaVariant::Undefiend(_) => {
                log::debug!("cell as undefiend: {cell_meta:?}");
                let props = yew::props! { undefiend::Props {
                    apply: cell_type_apply,
                }};
                html!{
                    <UndefiendComponent ..props/>
                }
            },
            CellMetaVariant::Value(value) => {
                log::debug!("cell as value: {cell_meta:?}");
                let props = yew::props! { value::Props {value: value.clone(), apply: value_apply} };
                html!{ 
                    <>
                    { header }
                    <ValueComponent ..props/> 
                    </>
                }                    
            },
            CellMetaVariant::Multystate(_) => {
                log::debug!("cell as multystae: {cell_meta:?}");
                let props = yew::props! { multystate::Props {edit_mode: *edit_mode} };
                html!{ 
                    <>
                    { header }
                    <MultystateComponent ..props/> 
                    </>
                }    
            },
            CellMetaVariant::Widget(_) => {
                log::debug!("cell as widget: {cell_meta:?}");
                let props = yew::props! { widget::Props { edit_mode: *edit_mode} };
                html!{
                    <>
                    { header }
                    <WidgetComponent ..props/> 
                    </>
                }                    
            },
        }
    };

    html! {
        <div>
            { details_vew }
        </div>
    }

}