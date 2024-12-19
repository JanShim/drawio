use common_model::{dflow_cell::{CellType, DFlowVariant}, geom_value::GeomValueXml, label_value::LabelValueXml, multystate::MultystateXml};
use yew::prelude::*;
use yew_hooks::{use_list, use_toggle, use_unmount};
use std::{cell::RefCell, collections::{HashMap, HashSet}, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, HtmlInputElement};

use crate::{
    components::{
        geom_value::GeomValue, label_value::LabelValueComponent,
        multystate::MultystateComponent, shared::{MdIcon, MdIconType},
        widget::WidgetContainer
    }, model::{cell_meta::{ form::CellDetailsForm, get_cellmeta_types, CellMeta, TypesItem, CELL_TYPE_GEOM, CELL_TYPE_LABEL, CELL_TYPE_MULTY },
    mx_cell::MxCell},
    store::cell::{self, cell_type_compare, CellInfoContext,
        SetCellTypeAction, StartApplyAction, NOT_CELL, NO_CONTEXT_FOUND
    },
};

#[function_component]
pub fn CellDetails() -> Html {
    use_unmount(|| {
        log::debug!("CellDetailsComponent unmount");
    });

    let context = &use_context::<CellInfoContext>().expect(NO_CONTEXT_FOUND);

    // let cell_meta_handler = use_state(|| {
    //         let cell_meta = context.mx_cell.get_meta().unwrap_or_default();
    //         let cell_types = cell_meta.types;

    //         let available = context.available_types;

    //         available.iter()

    //         log::debug!("{cell_types:?}");

    //     });

    let cell_types_list = use_list({
            let cell_meta = context.mx_cell.get_meta().unwrap_or_default();
            // let cell_types = get_cellmeta_types(&cell_meta.types);
            let cell_types = &cell_meta.types.iter()
                .map(|o| (o.get_cell_type(), o.clone()))
                .collect::<HashMap<CellType, DFlowVariant>>();

            context.available_types.iter()
                .map(|o: TypesItem| (o.clone(), cell_types.get(&o.cell_type).map(|i| i.clone())))
                .collect::<Vec<(TypesItem, Option<DFlowVariant>)>>()
        });

    // let (cell_state, cell_state_dispatch) = use_store::<cell::State>();
    // let cell_meta = use_mut_ref(|| cell_state.meta.clone());
    // let meta = use_state(|| cell_state.meta.clone());
    // {
    //     let meta = meta.clone();
    //     use_effect_with(cell_state.clone(), move |st| {
    //         // log::debug!("use_effect_with cell meta: {:?}", st.meta);
    //         meta.set(st.meta.clone());
    //     });
    // }

    let edit_mode = use_toggle(false, true);
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
            let edit_mode = edit_mode.clone();
            // let cell_state_dispatch = cell_state_dispatch.clone();
            Callback::from(move |_: MouseEvent| {
            //    cell_state_dispatch.apply(StartApplyAction(true));
                log::debug!("apply edited");
                edit_mode.toggle();
            })
        };

    // let features_set = use_mut_ref(|| cell_meta_handler.get_cell_type());
    let on_detals_apply = {
            let edit_mode = edit_mode.clone();
            // let features_set = features_set.clone();
            // let cell_meta = cell_meta_handler.clone();
            // let mx_editor = context.mx_editor;
            // let mx_cell = context.mx_cell;
            // let meta = meta.clone();
            Callback::from(move |variant: DFlowVariant| {
                todo!()
                // let cell_type = variant.get_cell_type();
                // log::debug!("my type: {cell_type:?}");

                // let mut new_meta = (*cell_meta).clone();
                // match variant {
                //     DFlowVariant::Label(value) => new_meta.set_label_meta(value),
                //     DFlowVariant::Multystate(value) => new_meta.set_multystate_meta(value),
                //     DFlowVariant::Geometry(value) => new_meta.set_geometry_meta(value),
                //     DFlowVariant::WidgetContainer(value) => new_meta.set_widget_container_meta(value),
                //     _ => (),
                // }

                // // *cell_meta.borrow_mut() = new_meta.clone();     // put to RefCell. Accumulate CellMetaVariant changes
                // // log::debug!("NEW_META: {:?}; CELL_META: {:?}", new_meta, cell_meta.borrow());

                // cell_meta.set(new_meta);         // set for redrawing curr component

                // log::debug!("apply set: {:?} -{cell_type:?}", features_set.borrow());
                // features_set.borrow_mut().remove(&cell_type);      // remove from set

                // // try to set meta in cell
                // if features_set.borrow().len() == 0 {
                //     // let cell_meta = cell_meta.borrow();      // get accumulated CellMetaVariants

                //     // let cell = cell_state.cell.clone().expect(NOT_CELL);

                //     // for widget container. Set widget selected widget to mxGraphModel
                //     if cell_type == CellType::WIDGETCONTAINER {
                //         // log::debug!("{}", cell_state.model_node.to_string());
                //         set_widget_model(&mx_editor, &mx_cell, cell_state.model_node.to_string());
                //     }

                //     log::debug!("set to cell: {cell_meta:?}");
                //     let _ = mx_cell.set_meta(&cell_meta).ok();

                //     // reset apply counter
                //     *features_set.borrow_mut() = cell_meta.get_cell_type();

                //     cell_state_dispatch.apply(StartApplyAction(false));
                //     edit_mode.set(false);
                // }
          })
        };

    let on_apply = {
        // let mx_graph_context = mx_graph_context.clone();
        let edit_mode = edit_mode.clone();
        // let info_form_handler = info_form_handler.clone();
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

                    log::debug!("{form:?}");

                }
            }

            // if let Some(form) = form {
            //     if let Some(form) = FormData::new_with_form(&form).ok().map(|data| Into::<DiagramForm>::into(data)) {
            //         let diagram_meta = &form.diagram_meta;

            //         log::debug!("CURR meta: {:?}", diagram_meta);

            //         // chage cell0 in graph model
            //         let diagram_meta_str = quick_xml::se::to_string(diagram_meta).unwrap();
            //         set_cell0_value(&mx_graph_context.mx_editor, diagram_meta_str);

            //         // store form meta to state
            //         info_form_handler.set(form.clone());

            //         // send to db
            //         let mx_graph_context = mx_graph_context.clone();
            //         let info_form_handler = info_form_handler.clone();
            //         let edit_mode = edit_mode.clone();
            //         wasm_bindgen_futures::spawn_local(async move {
            //             if let Ok(node) = mx_graph_context.get_graph_xml() {
            //                 if let Ok(Some(model_str)) = mx_graph_context.get_xml(node) {
            //                     let svg = mx_graph_context.get_graph_svg();

            //                     if form.is_new_item() {
            //                         let item = DiagramDto::new(
            //                                 form.name.to_string(),
            //                                 model_str,
            //                                 Some(svg),
            //                             );

            //                         let result = post(format!("{}/diagram", mx_graph_context.api_url), item).await;
            //                         match result {
            //                             Ok(created) => {
            //                                 log::debug!("created result: {:?}", created);
            //                                 apply_form_to_state(&created, &mx_graph_context.mx_editor, info_form_handler);
            //                             },
            //                             Err(err) => log::debug!("widget create error: {err:?}"),
            //                         }
            //                     } else {
            //                         let item = DiagramDto {
            //                                 uuid: form.uuid.to_string(),
            //                                 name: form.name.to_string(),
            //                                 model: model_str,
            //                                 svg: Some(svg),
            //                             };

            //                         let result = put(format!("{}/diagram/{}", mx_graph_context.api_url, form.uuid), item).await;
            //                         match result {
            //                             Ok(updated) => {
            //                                 log::debug!("updated result: {:?}", updated);
            //                                 apply_form_to_state(&updated, &mx_graph_context.mx_editor, info_form_handler);
            //                             },
            //                             Err(err) => log::debug!("widget {} update error: {err:?}", form.uuid),
            //                         }
            //                     }

            //                     // exit edit mode
            //                     edit_mode.set(false);
            //                 };
            //             }
            //         });

            //     }
            // }
        })};


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

    // let details_vew = {
    //     let edit_mode = edit_mode.clone();
    //     let cell_meta = cell_meta_handler.clone();
    //     todo!()
    //     // cell_meta.clone().types.iter()
    //     //     .map(|o| {
    //     //         match o.clone() {
    //     //             DFlowVariant::Undefiend(_) => {
    //     //                 log::debug!("cell type undefiend");
    //     //                 html!{ "Error cell type" }
    //     //             },
    //     //             DFlowVariant::Label(value) => {
    //     //                 log::debug!("cell as label: {value:?}");
    //     //                 html!{
    //     //                     <LabelValueComponent edit_mode={*edit_mode}
    //     //                         value={ value.clone() }
    //     //                         on_detals_apply={ on_detals_apply.clone() }/>
    //     //                 }
    //     //             },
    //     //             DFlowVariant::Multystate(value) => {
    //     //                 log::debug!("cell as multystate: {value:?}");
    //     //                 html!{
    //     //                     <MultystateComponent edit_mode={*edit_mode}
    //     //                         value={ value.clone() }
    //     //                         on_detals_apply={on_detals_apply.clone()}/>
    //     //                 }
    //     //             },
    //     //             DFlowVariant::WidgetContainer(value) => {
    //     //                 log::debug!("cell as widget container: {:?}", value);
    //     //                 html!{
    //     //                     <WidgetContainer edit_mode={*edit_mode}
    //     //                     value={ value.clone() }
    //     //                     on_detals_apply={on_detals_apply.clone()}/>
    //     //                 }
    //     //             },
    //     //             DFlowVariant::Geometry(value) => {
    //     //                 log::debug!("cell as geometry: {:?}", cell_meta);
    //     //                 html!{
    //     //                     <GeomValue edit_mode={*edit_mode}
    //     //                         value={ value.clone() }
    //     //                         on_detals_apply={ on_detals_apply.clone() }/>
    //     //                 }
    //     //             },
    //     //         }
    //     //     })
    //     //     .collect::<Vec<_>>()
    // };

    let details_view = {
            let edit_mode = edit_mode.clone();
            html! {<>
                // <input type="hidden" id="uuid" name="uuid" value={"aaaaaa"}/>
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

                            html!{ <LabelValueComponent edit_mode={*edit_mode} {value}/> }
                        },
                        CellType::MULTYSTATE => {
                            let value = variant.clone()
                                .map(|o| {
                                    match o {
                                        DFlowVariant::Multystate(value) => value,
                                        _ => { log::error!("not multystate cell variant"); MultystateXml::default()}
                                    }
                                });

                            html!{ <MultystateComponent edit_mode={*edit_mode} {value}/> }
                        },
                        CellType::GEOM => {
                            let value = variant.clone()
                                .map(|o| {
                                    match o {
                                        DFlowVariant::Geometry(value) => value,
                                        _ => { log::error!("not geometry cell variant"); GeomValueXml::default()}
                                    }
                                });

                            html!{ <GeomValue edit_mode={*edit_mode} {value}/> }
                        },
                        _ => html!(),
                    }
                })}
            </>}
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

#[function_component]
pub fn CellTypeSelector() -> Html
{
    use_unmount(|| {
        log::debug!("CellTypeSelectorComponent unmount");
    });

    let context = &use_context::<CellInfoContext>().expect(NO_CONTEXT_FOUND);

    // let (_, cell_state_dispatch) = use_store::<cell::State>();
    let cell_types: Rc<RefCell<Vec<TypesItem>>> = use_mut_ref(|| {
            vec![
                // TypesItem {name: CELL_TYPE_LABEL.into(), label: "Значение".into(), selected: false},
                // TypesItem {name: CELL_TYPE_MULTY.into(), label: "Множество состояний".into(), selected: false},
                // TypesItem {name: CELL_TYPE_GEOM.into(), label: "Геометрия".into(), selected: false},
            ]
        });

    let cell_types_map = use_mut_ref(|| HashMap::<String, CellType>::new());

    let is_checked = use_state(|| false);
    let is_checkable = use_state(|| false);

    let cell_types_apply: Callback<MouseEvent> = {
            let cell_types_map = cell_types_map.clone();
            let cell = context.mx_cell.clone();
            Callback::from(move |_: MouseEvent| {
                let cell_types = cell_types_map.borrow().values()
                    .map(|o| (*o).clone())
                    .collect::<HashSet<_>>();

                // cell_state_dispatch.apply(SetCellTypeAction(cell_types));
                let mut cell_types = cell_types.into_iter().collect::<Vec<_>>();
                cell_types.sort_by(cell_type_compare);

                let data = cell_types.into_iter()
                    .map(|o| Into::<DFlowVariant>::into(o) )
                    .collect::<Vec<_>>();

                // let cell = state.cell.clone().ok_or(JsValue::from(NOT_CELL)).unwrap();
                let mut meta = cell.get_meta().unwrap();
                meta.types = data;

                // // assigne meta to editor cell
                let res = cell.set_meta(&meta);
                if res.is_err() {
                    log::error!("{:?}", res.unwrap_err())
                }

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
