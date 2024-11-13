use yew::prelude::*;
use yew_hooks::use_unmount;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yewdux::use_store;

use crate::{
    components::{
        label_value::LabelValueComponent, multystate::MultystateComponent, shared::{MdIcon, MdIconType}, widget::WidgetComponent
    },
    model::cell_meta::{
         CellMetaVariant, CellType
    }, 
    store::cell::{self, SetCellTypeAction, StartApplyAction, NOT_CELL }, 
};

#[function_component]
pub fn CellDetailsComponent() -> Html {
    use_unmount(|| {
        log::debug!("CellDetailsComponent unmount");
    });    

    let (cell_state, cell_state_dispatch) = use_store::<cell::State>();
    let cell_meta = use_mut_ref(|| cell_state.meta.clone());

    let edit_mode = use_state(|| false);

    let edit_mode_toggle = {
            let edit_mode = edit_mode.clone();
            Callback::from(move |_: MouseEvent| { edit_mode.set(true); })
        };

    let cell_details_start: Callback<MouseEvent> = {
            let cell_state_dispatch = cell_state_dispatch.clone();
            Callback::from(move |_: MouseEvent| {
               cell_state_dispatch.apply(StartApplyAction(true));
            })
        };

    let features_set = use_mut_ref(|| cell_meta.borrow().get_cell_type());
    let on_detals_apply = {
            let edit_mode = edit_mode.clone();
            let features_set = features_set.clone();
            let cell_meta = cell_meta.clone();
            Callback::from(move |variant: CellMetaVariant| {
                let cell_type = match variant {
                        CellMetaVariant::Label(_) => CellType::LABEL,
                        CellMetaVariant::Multystate(_) => CellType::MULTYSTATE,
                        CellMetaVariant::WidgetContainer(_) => CellType::WIDGETCONTAINER,
                    };

                let mut new_meta = cell_meta.borrow().clone();
                match variant {
                    CellMetaVariant::Label(value) => new_meta.set_label_meta(value),
                    CellMetaVariant::Multystate(value) => new_meta.set_multystate_meta(value),
                    _ => (),
                }

                *cell_meta.borrow_mut() = new_meta;

                log::debug!("apply set: {:?} -{cell_type:?}", features_set.borrow());
                features_set.borrow_mut().remove(&cell_type);      // remove from set

                // try to set meta in cell
                if features_set.borrow().len() == 0 {
                    let meta = (*cell_meta.borrow()).clone();
                    log::debug!("apply meta to cell {meta:?}");
    
                    let cell = cell_state.cell.clone().expect(NOT_CELL);
    
                    // //TODO: забыл зачем это?
                    // set_widget_model(mx_graph_context.mx_editor.clone(), (*cell).clone(), cell_state.model_node.to_string());
    
                    // // let new_meta = (**meta).clone();
                    let _ = cell.set_meta(&meta).ok();
    
                    // reset apply counter
                    *features_set.borrow_mut() = meta.get_cell_type();
    
                    cell_state_dispatch.apply(StartApplyAction(false));
                    edit_mode.set(false);
                }                   
          })
        };


    // ============= views ================
    let header = {
            let header_props = yew::props! { CellDetailsHeaderProps {
                edit_mode: *edit_mode,
                cell_details_apply: cell_details_start,
                edit_mode_toggle,
            } };   

            html! {
                <CellDetailsHeader ..header_props />
            }
        };

    let details_vew = {
        let edit_mode = edit_mode.clone();
        let state_meta = cell_meta.clone();
        state_meta.clone().borrow().types.iter()
            .map(|o| {
                match o.clone() {
                    CellMetaVariant::Label(value) => {
                        log::debug!("cell as label: {:?}", state_meta);
                        html!{ 
                            <LabelValueComponent edit_mode={*edit_mode} 
                                value={ value.clone() } 
                                on_detals_apply={ on_detals_apply.clone() }/> 
                        }
                    },
                    CellMetaVariant::Multystate(_) => {
                        log::debug!("cell as multystate: {:?}", *state_meta);
                        html!{ 
                            <MultystateComponent edit_mode={*edit_mode} 
                                cell_meta={ state_meta.clone() }
                                on_detals_apply={on_detals_apply.clone()}/> 
                        }    
                    },
                    CellMetaVariant::WidgetContainer(_) => {
                        log::debug!("cell as widget: {:?}", *state_meta);
                        html!{
                            <WidgetComponent edit_mode={*edit_mode}/> 
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
pub fn CellTypeSelectorComponent() -> Html 
{
    use_unmount(|| {
        log::debug!("CellTypeSelectorComponent unmount");
    });

    let (_, cell_state_dispatch) = use_store::<cell::State>();
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
    pub cell_details_apply: Callback<MouseEvent>,
    pub edit_mode_toggle: Callback<MouseEvent>,
}


#[function_component]
pub fn CellDetailsHeader(CellDetailsHeaderProps { edit_mode, cell_details_apply, edit_mode_toggle }: &CellDetailsHeaderProps) -> Html {
    html!{
        <div class="flex-box-2 delim-label" >
        // arrow_back
        if *edit_mode {
            <button onclick={cell_details_apply}><MdIcon icon={MdIconType::Check}/></button>
        } else {
            <button onclick={edit_mode_toggle}><MdIcon icon={MdIconType::Edit}/></button>
        }
        </div>           
    }    
}
