use common_model::dflow_cell::{CellType, DFlowVariant};
use yew::prelude::*;
use yew_hooks::use_unmount;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yewdux::use_store;

use crate::{
    components::{
        geom_value::GeomValue, label_value::LabelValueComponent, 
        multystate::MultystateComponent, shared::{MdIcon, MdIconType}, 
        widget::WidgetContainer
    },
    model::cell_meta::{ CELL_TYPE_GEOM, CELL_TYPE_LABEL, CELL_TYPE_MULTY }, 
    store::{
        cell::{self, SetCellTypeAction, StartApplyAction, NOT_CELL, NO_CONTEXT_FOUND }, 
        mx_context::TMxGraphContext
    }, 
    utils::set_widget_model, 
};

#[function_component]
pub fn CellDetails() -> Html {
    use_unmount(|| {
        log::debug!("CellDetailsComponent unmount");
    });    

    let mx_graph_context = use_context::<TMxGraphContext>().expect(NO_CONTEXT_FOUND);

    let (cell_state, cell_state_dispatch) = use_store::<cell::State>();
    let cell_meta = use_mut_ref(|| cell_state.meta.clone());
    let meta = use_state(|| cell_state.meta.clone());
    {
        let meta = meta.clone();
        use_effect_with(cell_state.clone(), move |st| {
            // log::debug!("use_effect_with cell meta: {:?}", st.meta);
            meta.set(st.meta.clone());
        });
    }

    let edit_mode = use_state(|| false);

    let on_edit_mode_set = {
            let edit_mode = edit_mode.clone();
            Callback::from(move |_: MouseEvent| { 
                edit_mode.set(true); 
            })
        };

    let on_cancel = {
            let edit_mode = edit_mode.clone();
            Callback::from(move |_: MouseEvent| { 
                edit_mode.set(false); 
            })
        };        

    let cell_details_start: Callback<MouseEvent> = {
            let cell_state_dispatch = cell_state_dispatch.clone();
            Callback::from(move |_: MouseEvent| {
               cell_state_dispatch.apply(StartApplyAction(true));
            })
        };

    let features_set = use_mut_ref(|| meta.get_cell_type());
    let on_detals_apply = {
            let edit_mode = edit_mode.clone();
            let features_set = features_set.clone();
            let cell_meta = cell_meta.clone();
            let meta = meta.clone();
            Callback::from(move |variant: DFlowVariant| {
                let cell_type = variant.get_cell_type();

                // log::debug!("my type: {cell_type:?}");

                let mut new_meta = cell_meta.borrow().clone();
                match variant {
                    DFlowVariant::Label(value) => new_meta.set_label_meta(value),
                    DFlowVariant::Multystate(value) => new_meta.set_multystate_meta(value),
                    DFlowVariant::Geometry(value) => new_meta.set_geometry_meta(value),
                    DFlowVariant::WidgetContainer(value) => new_meta.set_widget_container_meta(value),
                    _ => (),
                }
                
                *cell_meta.borrow_mut() = new_meta.clone();     // put to RefCell. Accumulate CellMetaVariant changes
                // log::debug!("NEW_META: {:?}; CELL_META: {:?}", new_meta, cell_meta.borrow());
                
                meta.set(new_meta.clone());         // set for redrawing curr component

                log::debug!("apply set: {:?} -{cell_type:?}", features_set.borrow());
                features_set.borrow_mut().remove(&cell_type);      // remove from set

                // try to set meta in cell
                if features_set.borrow().len() == 0 {
                    let cell_meta = cell_meta.borrow();      // get accumulated CellMetaVariants
    
                    let cell = cell_state.cell.clone().expect(NOT_CELL);

                    // for widget container. Set widget selected widget to mxGraphModel
                    if cell_type == CellType::WIDGETCONTAINER {
                        // log::debug!("{}", cell_state.model_node.to_string());
                        set_widget_model(mx_graph_context.mx_editor.clone(), (*cell).clone(), cell_state.model_node.to_string());
                    }

                    log::debug!("set to cell: {cell_meta:?}");
                    let _ = cell.set_meta(&cell_meta).ok();
    
                    // reset apply counter
                    *features_set.borrow_mut() = cell_meta.get_cell_type();
    
                    cell_state_dispatch.apply(StartApplyAction(false));
                    edit_mode.set(false);
                }                   
          })
        };


    // ============= views ================
    let header = {
            let header_props = yew::props! { CellDetailsHeaderProps {
                edit_mode: *edit_mode,
                on_cell_details_apply: cell_details_start,
                on_edit_mode_set,
                on_cancel,
            } };   

            html! {
                <CellDetailsHeader ..header_props />
            }
        };

    let details_vew = {
        let edit_mode = edit_mode.clone();
        let cell_meta = meta.clone();
        cell_meta.clone().types.iter()
            .map(|o| {
                match o.clone() {
                    DFlowVariant::Undefiend(_) => {
                        log::debug!("cell type undefiend");
                        html!{ "Error cell type" }
                    },
                    DFlowVariant::Label(value) => {
                        log::debug!("cell as label: {value:?}");
                        html!{ 
                            <LabelValueComponent edit_mode={*edit_mode} 
                                value={ value.clone() }
                                on_detals_apply={ on_detals_apply.clone() }/> 
                        }
                    },
                    DFlowVariant::Multystate(value) => {
                        log::debug!("cell as multystate: {value:?}");
                        html!{ 
                            <MultystateComponent edit_mode={*edit_mode} 
                                value={ value.clone() }
                                on_detals_apply={on_detals_apply.clone()}/> 
                        }    
                    },
                    DFlowVariant::WidgetContainer(value) => {
                        log::debug!("cell as widget container: {:?}", value);
                        html!{
                            <WidgetContainer edit_mode={*edit_mode}
                            value={ value.clone() }
                            on_detals_apply={on_detals_apply.clone()}/> 
                        }                    
                    },
                    DFlowVariant::Geometry(value) => {
                        log::debug!("cell as geometry: {:?}", cell_meta);
                        html!{ 
                            <GeomValue edit_mode={*edit_mode} 
                                value={ value.clone() } 
                                on_detals_apply={ on_detals_apply.clone() }/> 
                        }
                    },                    
                }
            })
            .collect::<Vec<_>>()
    };

    html! {
        <div >
            { header }
            { details_vew }
        </div>
    }

}

// ----------------------------------------------
struct TypesItem {
    pub name: AttrValue,
    pub label: AttrValue,
    pub selected: bool,
}

#[function_component]
pub fn CellTypeSelector() -> Html 
{
    use_unmount(|| {
        log::debug!("CellTypeSelectorComponent unmount");
    });

    let (_, cell_state_dispatch) = use_store::<cell::State>();
    let cell_types = use_mut_ref(|| {
            vec![
                TypesItem {name: CELL_TYPE_LABEL.into(), label: "Значение".into(), selected: false},
                TypesItem {name: CELL_TYPE_MULTY.into(), label: "Множество состояний".into(), selected: false},
                TypesItem {name: CELL_TYPE_GEOM.into(), label: "Геометрия".into(), selected: false},
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
                    val if val == CELL_TYPE_LABEL => {
                        if checked {
                            cell_types_map.borrow_mut().insert(val, CellType::LABEL);
                        } else {
                            cell_types_map.borrow_mut().remove(&val);
                        }
                    },
                    val if val == CELL_TYPE_MULTY => {
                        if checked {
                            cell_types_map.borrow_mut().insert(val, CellType::MULTYSTATE);
                        } else {
                            cell_types_map.borrow_mut().remove(&val);
                        }
                    },
                    val if val == CELL_TYPE_GEOM => {
                        if checked {
                            cell_types_map.borrow_mut().insert(val, CellType::GEOM);
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
            <div class="flex-box-2 delim-label" >
                <button onclick={cell_types_apply} disabled={!*is_checkable}><MdIcon icon={MdIconType::Check}/></button>
            </div>   

            <fieldset class="types-list">
                <legend>{"Выберите нужные функции:"}</legend>
                { list_vew }
            </fieldset>                        
        </div>
    }
}

// ----------------------------------------------
#[derive(Properties, PartialEq, Debug)]
pub struct CellDetailsHeaderProps {
    pub edit_mode: bool,
    pub on_cell_details_apply: Callback<MouseEvent>,
    pub on_edit_mode_set: Callback<MouseEvent>,
    pub on_cancel: Callback<MouseEvent>,
}


#[function_component]
pub fn CellDetailsHeader(CellDetailsHeaderProps { 
    edit_mode, 
    on_cell_details_apply, 
    on_edit_mode_set,
    on_cancel,
}: &CellDetailsHeaderProps) -> Html {
    
    html!{
        <div class="flex-box-2 delim-label" >
        // arrow_back
        if *edit_mode {
            <div style="width:64px"> 
                <button onclick={on_cell_details_apply}><MdIcon icon={MdIconType::Check}/></button>
                <button onclick={on_cancel}><MdIcon icon={MdIconType::Cancel}/></button>
            </div>
        } else {
            <button onclick={on_edit_mode_set}><MdIcon icon={MdIconType::Edit}/></button>
        }
        </div>           
    }    
}
