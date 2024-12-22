use yew::prelude::*;
use yew_hooks::{use_list, use_toggle, use_unmount};
use std::{cell::RefCell, collections::{HashMap, HashSet}, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, HtmlInputElement};
use common_model::{dflow_cell::{CellType, DFlowVariant}, geom_value::GeomValueXml, label_value::LabelValueXml, multystate::MultystateXml};

use crate::{
    components::{
        geom_value::GeomValue, label_value::LabelValueComponent,
        multystate::MultystateComponent, shared::{MdIcon, MdIconType},
    },
    model::cell_meta::{ form::CellDetailsForm, TypesItem, CELL_TYPE_GEOM, CELL_TYPE_LABEL, CELL_TYPE_MULTY },
    store::cell::{
        cell_type_compare,
        CellInfoContext,
        NO_CONTEXT_FOUND
    },
};

#[function_component]
pub fn CellDetails() -> Html {
    use_unmount(|| {
        log::debug!("CellDetailsComponent unmount");
    });

    let context = &use_context::<CellInfoContext>().expect(NO_CONTEXT_FOUND);
    let force_updater = use_force_update();

    let cell_types_list = use_list({
            let cell_meta = context.mx_cell.get_meta().unwrap_or_default();

            let cell_types = &cell_meta.types.iter()
                .map(|o| (o.get_cell_type(), o.clone()))
                .collect::<HashMap<CellType, DFlowVariant>>();

            context.available_types.iter()
                .map(|o: TypesItem| (o.clone(), cell_types.get(&o.cell_type).map(|i| i.clone())))
                .collect::<Vec<(TypesItem, Option<DFlowVariant>)>>()
        });

    let edit_mode = use_toggle(false, true);
    let on_edit_mode_set = {
            let edit_mode = edit_mode.clone();
            Callback::from(move |_: MouseEvent| {
                edit_mode.set(true);
            })
        };

    let on_cancel = {
            let edit_mode = edit_mode.clone();
            let force_updater = force_updater.clone();
            Callback::from(move |_: MouseEvent| {
                edit_mode.set(false);
                force_updater.force_update();
            })
        };

    let on_apply = {
        let context = context.clone();
        let edit_mode = edit_mode.clone();
        // let force_updater = force_updater.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let form = event.target()
                .and_then(|t| {
                    match t.dyn_into::<HtmlFormElement>() {
                        Ok(el) => Some(el),
                        Err(err) => { log::error!("{err:?}"); None },
                    }
                });

            if let Some(form) = form {
                if let Some(form) = FormData::new_with_form(&form).ok().map(|data| Into::<CellDetailsForm>::into(data)) {
                    let mut meta = context.mx_cell.get_meta().unwrap_or_default();
                    meta.types = form.variants;

                    // apply meta to cell
                    let tst = context.mx_cell.set_meta(&meta);
                    if tst.is_err() {
                        log::error!("{:?}", tst.unwrap_err());
                    }


                    // close edit mode
                    edit_mode.toggle();
                }
            }
        })};

    // ============= views ================
    let header = {
            let header_props = yew::props! { CellDetailsHeaderProps {
                edit_mode: *edit_mode,
                on_edit_mode_set,
                on_cancel,
            } };

            html! {
                <CellDetailsHeader ..header_props />
            }
        };

    let details_view = {
            let edit_mode = edit_mode.clone();
                if *edit_mode {
                    html! {<>
                        {for cell_types_list.current().iter()
                            .map(|(item, variant)| {
                                match item.cell_type {
                                    CellType::LABEL => {
                                        let value = variant.clone()
                                            .map(|o| {
                                                match o {
                                                    DFlowVariant::Label(value) => value,
                                                    _ => { log::error!("not label cell variant"); LabelValueXml::default()}
                                                }
                                            });

                                        html!{ <LabelValueComponent edit_mode={ true } {value}/> }
                                    },
                                    CellType::MULTYSTATE => {
                                        let value = variant.clone()
                                            .map(|o| {
                                                match o {
                                                    DFlowVariant::Multystate(value) => value,
                                                    _ => { log::error!("not multystate cell variant"); MultystateXml::default()}
                                                }
                                            });

                                        html!{ <MultystateComponent edit_mode={ true } {value}/> }
                                    },
                                    CellType::GEOM => {
                                        let value = variant.clone()
                                            .map(|o| {
                                                match o {
                                                    DFlowVariant::Geometry(value) => value,
                                                    _ => { log::error!("not geometry cell variant"); GeomValueXml::default()}
                                                }
                                            });

                                        html!{ <GeomValue edit_mode={ true } {value}/> }
                                    },
                                    _ => html!(),
                                }
                            })
                        }
                    </>}
                } else {
                    let current_variants = cell_types_list.current().iter()
                        .filter(|(_, variant)| variant.is_some())
                        .map(|(_, variant)| variant.clone().unwrap())
                        .collect::<Vec<_>>();

                    html! {<>
                        if current_variants.len() == 0 {
                            <h3>
                                { "Тип элемента не определен." }<br/>
                                { "Перейдите в режим редактирования." }
                            </h3>
                            //TODO: здесь можно попросить сбросить dflow
                        }
                        {for current_variants.into_iter()
                            .map(|variant| {
                                match variant {
                                    DFlowVariant::Label(value) => html!{
                                        <LabelValueComponent edit_mode={ false } {value}/>
                                    },
                                    DFlowVariant::Multystate(value) => html!{
                                        <MultystateComponent edit_mode={ false } {value}/>
                                    },
                                    DFlowVariant::Geometry(value) => html!{
                                        <GeomValue edit_mode={ false } {value}/>
                                    },
                                    _ => html! { <h1>{ "Тип элемента не обрабатывается" }</h1> }
                                }
                            })
                        }
                    </>}
                }
        };

    html! {
        <div >
            <form onsubmit={on_apply}>
                { header }
                { details_view }
            </form>
        </div>
    }

}

// ----------------------------------------------

// #[function_component]
// pub fn CellTypeSelector() -> Html
// {
//     use_unmount(|| {
//         log::debug!("CellTypeSelectorComponent unmount");
//     });

//     let context = &use_context::<CellInfoContext>().expect(NO_CONTEXT_FOUND);

//     // let (_, cell_state_dispatch) = use_store::<cell::State>();
//     let cell_types: Rc<RefCell<Vec<TypesItem>>> = use_mut_ref(|| {
//             vec![
//                 // TypesItem {name: CELL_TYPE_LABEL.into(), label: "Значение".into(), selected: false},
//                 // TypesItem {name: CELL_TYPE_MULTY.into(), label: "Множество состояний".into(), selected: false},
//                 // TypesItem {name: CELL_TYPE_GEOM.into(), label: "Геометрия".into(), selected: false},
//             ]
//         });

//     let cell_types_map = use_mut_ref(|| HashMap::<String, CellType>::new());

//     let is_checked = use_state(|| false);
//     let is_checkable = use_state(|| false);

//     let cell_types_apply: Callback<MouseEvent> = {
//             let cell_types_map = cell_types_map.clone();
//             let cell = context.mx_cell.clone();
//             Callback::from(move |_: MouseEvent| {
//                 let cell_types = cell_types_map.borrow().values()
//                     .map(|o| (*o).clone())
//                     .collect::<HashSet<_>>();

//                 // cell_state_dispatch.apply(SetCellTypeAction(cell_types));
//                 let mut cell_types = cell_types.into_iter().collect::<Vec<_>>();
//                 cell_types.sort_by(cell_type_compare);

//                 let data = cell_types.into_iter()
//                     .map(|o| Into::<DFlowVariant>::into(o) )
//                     .collect::<Vec<_>>();

//                 // let cell = state.cell.clone().ok_or(JsValue::from(NOT_CELL)).unwrap();
//                 let mut meta = cell.get_meta().unwrap();
//                 meta.types = data;

//                 // // assigne meta to editor cell
//                 let res = cell.set_meta(&meta);
//                 if res.is_err() {
//                     log::error!("{:?}", res.unwrap_err())
//                 }

//                 is_checked.set(true);
//             })
//         };

//     let onchange = {
//             let cell_types = cell_types.clone();
//             let cell_types_map = cell_types_map.clone();
//             let is_checkable = is_checkable.clone();
//             Callback::from(move |e: Event| {
//                 let target = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
//                 let checked = target.checked();
//                 let id = target.id();
//                 match id.clone() {
//                     val if val == CELL_TYPE_LABEL => {
//                         if checked {
//                             cell_types_map.borrow_mut().insert(val, CellType::LABEL);
//                         } else {
//                             cell_types_map.borrow_mut().remove(&val);
//                         }
//                     },
//                     val if val == CELL_TYPE_MULTY => {
//                         if checked {
//                             cell_types_map.borrow_mut().insert(val, CellType::MULTYSTATE);
//                         } else {
//                             cell_types_map.borrow_mut().remove(&val);
//                         }
//                     },
//                     val if val == CELL_TYPE_GEOM => {
//                         if checked {
//                             cell_types_map.borrow_mut().insert(val, CellType::GEOM);
//                         } else {
//                             cell_types_map.borrow_mut().remove(&val);
//                         }
//                     },
//                     _ => (),
//                 };
//                 is_checkable.set(cell_types_map.borrow().len() > 0);

//                 cell_types.borrow_mut().iter_mut()
//                     .for_each( |o| {
//                         if o.name.eq(&id) {
//                             o.selected = checked;
//                         }
//                     }) ;
//             })
//         };

//     // ============= views ================
//     let list_vew = {
//             cell_types.borrow().iter()
//                 .map(|o| {
//                     html! {
//                         <div>
//                             <input type="checkbox" id={o.name.clone()} name={o.name.clone()} checked={o.selected} onchange={onchange.clone()}/>
//                             <label for={o.name.clone()}>{o.label.clone()}</label>
//                         </div>
//                     }
//                 })
//                 .collect::<Html>()
//         };

//     html! {
//         <div>
//             <div class="flex-box-2 delim-label" >
//                 <button onclick={cell_types_apply} disabled={!*is_checkable}><MdIcon icon={MdIconType::Check}/></button>
//             </div>

//             <fieldset class="types-list">
//                 <legend>{"Выберите нужные функции:"}</legend>
//                 { list_vew }
//             </fieldset>
//         </div>
//     }
// }

// ----------------------------------------------
#[derive(Properties, PartialEq, Debug)]
pub struct CellDetailsHeaderProps {
    pub edit_mode: bool,
    // pub on_cell_details_apply: Callback<MouseEvent>,
    pub on_edit_mode_set: Callback<MouseEvent>,
    pub on_cancel: Callback<MouseEvent>,
}


#[function_component]
pub fn CellDetailsHeader(CellDetailsHeaderProps {
    edit_mode,
    // on_cell_details_apply,
    on_edit_mode_set,
    on_cancel,
}: &CellDetailsHeaderProps) -> Html {

    html!{
        <div class="flex-box-2 delim-label" >
        // arrow_back
        if *edit_mode {
            <div style="width:64px">
                // <button onclick={on_cell_details_apply}><MdIcon icon={MdIconType::Check}/></button>
                <button type="submit"><MdIcon icon={MdIconType::Check}/></button>
                <button onclick={on_cancel}><MdIcon icon={MdIconType::Cancel}/></button>
            </div>
        } else {
            <button onclick={on_edit_mode_set}><MdIcon icon={MdIconType::Edit}/></button>
        }
        </div>
    }
}
