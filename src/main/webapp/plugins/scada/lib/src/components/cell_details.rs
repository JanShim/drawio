use yew::prelude::*;
use yewdux::{use_selector, use_store};
use common_model::free_value::FreeValueXml;

use crate::{
    components::{
        multystate::{self, MultystateComponent}, 
        undefiend::{self, UndefiendComponent}, 
        value::{self, ValueComponent}, 
        widget::{self, WidgetComponent}
    }, model::cell_meta::{
        value_reducers::ApplyValueMetaAction, 
        CellMetaVariant, 
        CellType
    }, store::cell::{self, SetCellTypeAction}, utils::set_widget_model 
};


#[function_component(CellDetailsComponent)]
pub fn component() -> Html {
    let (cell_state, cell_state_dispatch) = use_store::<cell::State>();
    let cell_meta = use_selector(|cell_state: &cell::State| cell_state.meta.clone());
    use_effect_with(cell_meta.clone(), |meta| {
        log::debug!("use_effect_with: cell_meta {meta:?}");
    });

    let edit_mode = use_state(|| false);

    let value_apply = cell_state_dispatch.apply_callback(|value: FreeValueXml| ApplyValueMetaAction(value));

    let edit_mode_toggle = {
            let edit_mode = edit_mode.clone();
            Callback::from(move |_: MouseEvent| { edit_mode.set(true); })
        };

    let cell_details_apply: Callback<MouseEvent> = {
        let edit_mode = edit_mode.clone();
        let cell_state = cell_state.clone();
        let cell_meta = cell_meta.clone();
        Callback::from(move |_: MouseEvent| {
            log::debug!("CURR CELL META:: {:?}", cell_meta);
            set_widget_model(cell_state.mx_editor.clone(), cell_state.cell.clone(), cell_state.model_node.to_string());
            let _meta = cell_state.cell.set_meta(&cell_meta).ok();
            edit_mode.set(false);
        })
    };

    let cell_type_apply = {
        let cell_state_dispatch = cell_state_dispatch.clone();
        Callback::from(move |cell_type: CellType| {
            cell_state_dispatch.apply(SetCellTypeAction(cell_type));
        })
    };

    // let widget_apply = {
    //     let cell_meta = cell_meta.clone();
    //     Callback::from(move |widget_meta: WidgetMeta| {
    //         log::debug!("widget_apply {widget_meta:?}");
    //         cell_meta.clone().reduce(cell_meta::Action::SetWidgetMeta(widget_meta));
    //     })
    // };    

    // ============= views ================
    let details_vew = {
        let edit_mode = edit_mode.clone();
        let header_props = yew::props! { CellDetailsHeaderProps {
            edit_mode: *edit_mode,
            cell_details_apply,
            edit_mode_toggle,
        } };            
        match &cell_meta.data {
            CellMetaVariant::Undefiend(_) => {
                log::debug!("cell as undefiend: {cell_meta:?}");
                let props = yew::props! { undefiend::Props { apply: cell_type_apply, }};
                html!{
                    <UndefiendComponent ..props/>
                }
            },
            CellMetaVariant::Value(value) => {
                log::debug!("cell as value: {cell_meta:?}");
                let props = yew::props! { value::Props {value: value.clone(), apply: value_apply} };
                html!{ 
                    <>
                    <CellDetailsHeader ..header_props />
                    <ValueComponent ..props/> 
                    </>
                }                    
            },
            CellMetaVariant::Multystate(_) => {
                log::debug!("cell as multystate: {cell_meta:?}");
                let props = yew::props! { multystate::Props {edit_mode: *edit_mode} };
                html!{ 
                    <>
                    <CellDetailsHeader ..header_props />
                    <MultystateComponent ..props/> 
                    </>
                }    
            },
            CellMetaVariant::Widget(_) => {
                log::debug!("cell as widget: {cell_meta:?}");
                let props = yew::props! { widget::Props { edit_mode: *edit_mode }};
                html!{
                    <>
                    <CellDetailsHeader ..header_props />
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

#[derive(Properties, PartialEq, Debug)]
pub struct CellDetailsHeaderProps {
    pub edit_mode: bool,
    pub cell_details_apply: Callback<MouseEvent>,
    pub edit_mode_toggle: Callback<MouseEvent>,
}


#[function_component(CellDetailsHeader)]
pub fn cell_details_header(CellDetailsHeaderProps { edit_mode, cell_details_apply, edit_mode_toggle }: &CellDetailsHeaderProps) -> Html {
    html!{
        <div class="flex-box-2 delim-label" >
        if *edit_mode {
            <button onclick={cell_details_apply}><img src="images/checkmark.gif" width="16" height="16"/></button>
        } else {
            <button onclick={edit_mode_toggle}><img src="images/edit16.png"/></button>
        }
        </div>           
    }    
}