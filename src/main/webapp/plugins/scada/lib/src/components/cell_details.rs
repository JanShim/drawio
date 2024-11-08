use yew::prelude::*;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yewdux::{use_selector, use_store};
use common_model::free_value::LabelValueXml;

use crate::{
    components::{
          label_value::LabelValueComponent, multystate::MultystateComponent, widget::WidgetComponent
    }, model::cell_meta::{
        value_reducers::ApplyLabelValueMetaAction, 
        CellMetaVariant, 
        CellType
    }, store::cell::{self, SetCellTypeAction, StartApplyAction}, utils::set_widget_model
};


#[function_component]
pub fn CellDetailsComponent() -> Html {
    let (cell_state, cell_state_dispatch) = use_store::<cell::State>();
    let cell_meta = use_selector(|cell_state: &cell::State| cell_state.meta.clone());

    let edit_mode = use_state(|| false);

    let edit_mode_toggle = {
            let edit_mode = edit_mode.clone();
            Callback::from(move |_: MouseEvent| { edit_mode.set(true); })
        };

    let cell_details_apply: Callback<MouseEvent> = {

            let cell_state_dispatch = cell_state_dispatch.clone();
            Callback::from(move |_: MouseEvent| {
               cell_state_dispatch.apply(StartApplyAction(true));
            })
        };

    let features_count = use_mut_ref(|| cell_meta.types.len());
    let on_detals_apply = {
            let features_count = features_count.clone();
            Callback::from(move |_: bool| {
                *features_count.borrow_mut() -= 1;      // decrement counter
            })
        };

    // effect on cell_meta changed
    {
        let features_count = features_count.clone(); 
        let edit_mode = edit_mode.clone();
        let cell_state = cell_state.clone();
        let cell_state_dispatch = cell_state_dispatch.clone();
        use_effect_with(cell_meta.clone(), move |meta| {
            if *features_count.borrow() == 0 {
                log::debug!("use_effect_with: apply meta {meta:?}");

                //TODO: забыл чаем этоы
                set_widget_model(cell_state.mx_editor.clone(), cell_state.cell.clone(), cell_state.model_node.to_string());

                let new_meta = (**meta).clone();
                let _ = cell_state.cell.set_meta(&new_meta).ok();                

                cell_state_dispatch.apply(StartApplyAction(false));
                edit_mode.set(false);
            }            
        })
    };        

    // let cell_type_apply = {
    //     let cell_state_dispatch = cell_state_dispatch.clone();
    //     Callback::from(move |cell_type: CellType| {
    //         cell_state_dispatch.apply(SetCellTypeAction(cell_type));
    //     })
    // };

    // let widget_apply = {
    //     let cell_meta = cell_meta.clone();
    //     Callback::from(move |widget_meta: WidgetMeta| {
    //         log::debug!("widget_apply {widget_meta:?}");
    //         cell_meta.clone().reduce(cell_meta::Action::SetWidgetMeta(widget_meta));
    //     })
    // };    

    // ============= views ================
    let header = {
            let header_props = yew::props! { CellDetailsHeaderProps {
                edit_mode: *edit_mode,
                cell_details_apply,
                edit_mode_toggle,
            } };   

            html! {
                <CellDetailsHeader ..header_props />
            }
        };

    let details_vew = {
        let edit_mode = edit_mode.clone();
        cell_meta.types.iter()
            .map(|o| {
                match o.clone() {
                    CellMetaVariant::Label(value) => {
                        log::debug!("cell as label: {cell_meta:?}");
                        let label_value_apply = cell_state_dispatch
                            .apply_callback(|value: LabelValueXml| ApplyLabelValueMetaAction(value));  

                        html!{ 
                            <LabelValueComponent value={value.clone()} apply={label_value_apply} on_detals_apply={on_detals_apply.clone()}/> 
                        }
                    },
                    CellMetaVariant::Multystate(_) => {
                        log::debug!("cell as multystate: {cell_meta:?}");
                        html!{ 
                            <MultystateComponent edit_mode={*edit_mode} on_detals_apply={on_detals_apply.clone()}/> 
                        }    
                    },
                    CellMetaVariant::WidgetContainer(_) => {
                        log::debug!("cell as widget: {cell_meta:?}");
                        html!{
                            <WidgetComponent edit_mode={*edit_mode}/> 
                        }                    
                    },
                }
            })
            .collect::<Vec<_>>()
    };

    html! {
        <div>
            { header }
            { details_vew }
        </div>
    }

}

// ----------------------------------------------
#[function_component]
pub fn CellTypeSelectorComponent() -> Html {

    struct TypesItem {
        pub name: AttrValue,
        pub label: AttrValue,
        pub selected: bool,
    }

    let (_, cell_state_dispatch) = use_store::<cell::State>();
    // let cell_meta = use_selector(|cell_state: &cell::State| cell_state.meta.clone());

    let cell_types = use_mut_ref(|| {
            vec![
                TypesItem {name: "value".into(), label: "Значение".into(), selected: false},
                TypesItem {name: "multy".into(), label: "Множество состояний".into(), selected: false},
            ]            
        });

    let cell_types_map = use_mut_ref(|| HashMap::<String, CellType>::new());
    
    let is_checked = use_state(|| false);
    let is_checkable = use_state(|| false);

    let cell_types_apply: Callback<MouseEvent> = {
            let cell_types_map = cell_types_map.clone();
            Callback::from(move |_: MouseEvent| {
                let cell_types = cell_types_map.borrow().values()
                    .map(|o| (*o).clone())
                    .collect::<HashSet<_>>();

                cell_state_dispatch.apply(SetCellTypeAction(cell_types));
                is_checked.set(true);
            })
        };

    let onchange = {
            let cell_types = cell_types.clone();
            let cell_types_map = cell_types_map.clone();
            let is_checkable = is_checkable.clone();
            Callback::from(move |e: Event| {
                let target = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                let checked = target.checked();
                let id = target.id();
                match id.clone() {
                    val if val == "value" => {
                        if checked {
                            cell_types_map.borrow_mut().insert(val, CellType::LABEL);
                        } else {
                            cell_types_map.borrow_mut().remove(&val);
                        }
                    },
                    val if val == "multy" => {
                        if checked {
                            cell_types_map.borrow_mut().insert(val, CellType::MULTYSTATE);
                        } else {
                            cell_types_map.borrow_mut().remove(&val);
                        }
                    },
                    _ => (),
                };
                is_checkable.set(cell_types_map.borrow().len() > 0);

                cell_types.borrow_mut().iter_mut()
                    .for_each( |o| {
                        if o.name.eq(&id) {
                            o.selected = checked;
                        } 
                    }) ;
            })
        };

    // ============= views ================
    let list_vew = {
            cell_types.borrow().iter()
                .map(|o| {
                    html! {
                        <div>
                            <input type="checkbox" id={o.name.clone()} name={o.name.clone()} checked={o.selected} onchange={onchange.clone()}/>
                            <label for={o.name.clone()}>{o.label.clone()}</label>
                        </div>
                    }
                })
                .collect::<Html>()
        };

    html! {
        <div>
            // <CellTypeSelectorHeader cell_types_apply={cell_types_apply} />
            <div class="flex-box-2 delim-label" >
                <button onclick={cell_types_apply} disabled={!*is_checkable}><img src="images/checkmark.gif" width="16" height="16"/></button>
            </div>   

            <fieldset class="types-list">
                <legend>{"Выберите нужные функции:"}</legend>
                { list_vew }
            </fieldset>                        

            // { details_vew }
        </div>
    }
}

// ----------------------------------------------
#[derive(Properties, PartialEq, Debug)]
pub struct CellDetailsHeaderProps {
    pub edit_mode: bool,
    pub cell_details_apply: Callback<MouseEvent>,
    pub edit_mode_toggle: Callback<MouseEvent>,
}


#[function_component]
pub fn CellDetailsHeader(CellDetailsHeaderProps { edit_mode, cell_details_apply, edit_mode_toggle }: &CellDetailsHeaderProps) -> Html {
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
