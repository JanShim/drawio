use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::{FormData, HtmlFormElement};
use yew_hooks::{use_async_with_options, use_list, use_toggle, use_unmount, UseAsyncOptions, UseListHandle};
use yewdux::{use_selector, use_store, Dispatch};
use common_model::diagram::{WidgetPropertyXml, WidgetXml};

use crate::{
    components::shared::{use_list_selected, use_state_with, MdIcon, MdIconType}, model::{
        common::{DiagramMeta, GraphModel, ModelForm}, mx_cell::CellValue, mx_editor::MxEditor, widget::{form::WidgetForm, WidgetDto}, widget_group::WidgetGroupListItemDto
    },
    store::{cell::NO_CONTEXT_FOUND, diagram::{self, State}, mx_context::TMxGraphContext},
    utils::{cliped_model_box, fetch, get_cell0, get_cell0_meta, post, put, set_cell0_value}
};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub form: WidgetForm,
}

#[function_component]
pub fn WidgetInfoComponent(Props{ form }: &Props) -> Html {
    let mx_graph_context = use_context::<TMxGraphContext>().expect(NO_CONTEXT_FOUND);
    use_unmount(|| { log::debug!("unmount WidgetInfoComponent") });

    let edit_mode = use_toggle(false, true);

    let widget_props_list_handler = use_list(Vec::<WidgetPropertyXml>::new());

    let info_form_handler = {
        let editor = &mx_graph_context.mx_editor;
        let widget_props_list_handler = widget_props_list_handler.clone();
        let mut form = form.clone();
        use_state(move || {
            // decorate meta from cell0
            decorate_with_cell0_meta(&editor, &mut form, widget_props_list_handler);

            log::debug!("from in use_state:: {form:?}");

            //result
            form
        })};


    let widget_groups_list = {
        let url = mx_graph_context.api_url.clone();
        use_async_with_options(
            async move { fetch::<Vec::<WidgetGroupListItemDto>>(format!("{}/widget-group/list", url)).await },
            UseAsyncOptions::enable_auto(),
        )};

    // {
    //     let editor = mx_graph_context.mx_editor.clone();
    //     let widget_props_list_handler = widget_props_list_handler.clone();
    //     let dispatch = dispatch.clone();
    //     use_effect_with(redraw.clone(), move |redraw| {
    //         if **redraw {
    //             let form = recreate_meta(&editor, widget_props_list_handler);
    //             dispatch.reduce_mut(|state| {
    //                 state.redraw = false;
    //                 state.model_meta = ModelForm::Widget(form);
    //             });
    //         }
    //     })
    // }

    // let widget_form = {
    //     let editor = &mx_graph_context.mx_editor;
    //     let widget_props_list_handler = widget_props_list_handler.clone();
    //     let form_meta = form_meta;
    //     use_state(move || {
    //         let form = get_widget_form_meta(editor, form_meta, widget_props_list_handler);
    //         log::debug!("use_state::: {form:?}");
    //         form
    //     })
    // };
    // {
    //     let widget_form = widget_form.clone();
    //     use_effect_with((*form_meta).clone(), move |form| {
    //         log::debug!("use_effect_with::: {form:?}");
    //         widget_form.set((*form).clone());
    //     })
    // }


    // {
    //     let widget_form = widget_form.clone();
    //     use_effect_with(state.model_meta.clone(), move |model_meta| {
    //         match model_meta.clone() {
    //             ModelForm::Widget(mut form) => {
    //                 // if let Ok(CellValue::Object(el)) = get_cell0(editor).get_value() {
    //                 //     if let Ok(meta) = quick_xml::de::from_str::<WidgetXml>(el.inner_html().as_str()) {
    //                 //         // appy meta to store
    //                 //         let mut form = form.clone();
    //                 //         form.meta = el.outer_html().into();     // this is cell0 value
    //                 //         widget_props_list_handler.set(meta.property);       // state widget properties
    //                 //         widget_meta.set(form);
    //                 //     }
    //                 // } else {
    //                 //     log::debug!("else CellValue: {:?}", get_cell0(editor).get_value().unwrap());
    //                 // }
    //                 form.meta = widget_form.meta.clone();
    //                 log::debug!("NEW WIDGET!! {form:?}");
    //                 widget_form.set(form);  // store to state
    //             },
    //             _ => {
    //                 log::info!("this is not widget item");
    //             },
    //         }
    //     })
    // }

    // ============= efects =========================


    // ============= events ====================
    let edit_mode_toggle = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |_| { edit_mode.toggle(); })
    };

    let on_cancel = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |_| { edit_mode.set(false); })
    };

    let on_apply = {
        let mx_graph_context = mx_graph_context.clone();
        let edit_mode = edit_mode.clone();
        let info_form_handler = info_form_handler.clone();
        let widget_props_list_handler = widget_props_list_handler.clone();
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
                if let Some(form) = FormData::new_with_form(&form).ok().map(|data| Into::<WidgetForm>::into(data)) {
                    let diagram_meta = &form.diagram_meta;

                    log::debug!("CURR meta: {:?}", diagram_meta);

                    // chage cell0 in graph model
                    let diagram_meta_str = quick_xml::se::to_string(diagram_meta).unwrap();
                    set_cell0_value(&mx_graph_context.mx_editor, diagram_meta_str);

                    // ctore new property list
                    if let GraphModel::Widget(widget) = diagram_meta.model.clone() {
                        widget_props_list_handler.set(widget.property);
                    }

                    // store form meta to state
                    info_form_handler.set(form.clone());

                    // send to db
                    let mx_graph_context = mx_graph_context.clone();
                    let info_form_handler = info_form_handler.clone();
                    let edit_mode = edit_mode.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Ok(node) = mx_graph_context.get_graph_xml() {
                            if let Ok(Some(model_str)) = mx_graph_context.get_xml(node) {
                                let svg = mx_graph_context.get_graph_svg();
                                let model: String = cliped_model_box(model_str).into();

                                if form.is_new_item() {
                                    let item = WidgetDto::new(
                                            form.group.to_string(),
                                            form.name.to_string(),
                                            model,
                                            vec![],
                                            Some(svg)
                                        );

                                    let result = post(format!("{}/widget", mx_graph_context.api_url), item).await;
                                    match result {
                                        Ok(created) => {
                                            log::debug!("created reult: {:?}", created);
                                            apply_form_to_state(&created, &mx_graph_context.mx_editor, info_form_handler);
                                        },
                                        Err(err) => log::debug!("widget create error: {err:?}"),
                                    }
                                } else {
                                    let item = WidgetDto {
                                            uuid: form.uuid.to_string(),
                                            group: form.group.to_string(),
                                            name: form.name.to_string(),
                                            model,
                                            //TODO: разобраться с типами
                                            types: vec!["ZDV2".to_owned()],
                                            svg: Some(svg),
                                        };

                                    let result = put(format!("{}/widget/{}", mx_graph_context.api_url, form.uuid), item).await;
                                    match result {
                                        Ok(updated) => {
                                            log::debug!("updated reult: {:?}", updated);
                                            apply_form_to_state(&updated, &mx_graph_context.mx_editor, info_form_handler);
                                        },
                                        Err(err) => log::debug!("widget {} update error: {err:?}", form.uuid),
                                    }
                                }

                                // exit edit mode
                                edit_mode.set(false);
                            };
                        }
                    });
                }
            }
        })
    };

    let on_property_add = {
            let widget_props_list_handler = widget_props_list_handler.clone();
            Callback::from(move |event: MouseEvent| {
                event.prevent_default();
                widget_props_list_handler.push(WidgetPropertyXml::default());
        })};

    // ================= views =====================
    let header = html!{
            <div class="flex-box-2 delim-label" >
            if !*edit_mode {
                <button onclick={edit_mode_toggle}><MdIcon icon={MdIconType::Edit}/></button>
            }
            </div>
        };

    let wgroups_select = {
            let info_form_handler = info_form_handler.clone();
            if widget_groups_list.loading {
                html! {  }
            } else  {
                let selected_group =  info_form_handler.group.clone();
                widget_groups_list.data.as_ref().map_or_else(
                    || html! {},        // default
                    |data| html! {
                        <select name="group" class="input-100">
                            <option value="undef"></option>
                            {for data.iter().map(|item| {
                                let selected = item.pk == selected_group;
                                html!{ <option value={ item.pk.clone() }  {selected}>{ item.name.clone() }</option> }
                            })}
                        </select>
                })
            }
        };

    let form_view = {
            let info_form_handler = info_form_handler.clone();
            let diagram_meta = quick_xml::se::to_string(&info_form_handler.diagram_meta).unwrap_or_default();

            html! {
                <form onsubmit={on_apply}>
                    <input type="hidden" name="uuid" value={ info_form_handler.uuid.clone() }/>
                    <input type="hidden" name="meta" value={ diagram_meta.clone() }/>

                    <div class="label"><label for="uuid">{ "uuid: " }</label></div>
                    <input name="uuid-0" value={ format!("{}", info_form_handler.uuid) } disabled={true} class="input-100"/><br/>
                    <div class="label"><label for="name">{ "name: " }</label></div>
                    <input name="name" value={ format!("{}", info_form_handler.name) } class="input-100"/><br/>
                    <div class="label"><label for="group">{ "group: " }</label></div>
                    { wgroups_select }

                    <div>
                        <div class="label"><label for="props">{ "свойства виджета: " }</label></div>
                        <div class="flex-box delim-label"><button onclick={on_property_add}><MdIcon icon={MdIconType::Add}/></button></div>
                        <table class="prop-table">
                            <colgroup>
                                <col style="width: 30%"/>
                                <col style="width: 70%"/>
                            </colgroup>
                            {for widget_props_list_handler.current().iter().map(|item| {
                                html!{
                                    <tr>
                                        <td><input name="props-name" value={ format!("{}", item.name) } class="input-100"/></td>
                                        <td><input name="props-value" value={ format!("{}", item.ds.tag) } class="input-100"/></td>
                                    </tr>
                                }
                            })}
                        </table>
                    </div>

                    <div class="flex-box-2" >
                        <button type="button" onclick={on_cancel}>{"Cancel"}</button>
                        <button type="submit">{"Save"}</button>
                    </div>
                </form>
            }
        };

    let view = {
            let info_form_handler = info_form_handler.clone();
            html! {
                <>
                <div>
                    <div class="label">{ "uuid: " }</div>
                    <div class="value">{ format!("{}", info_form_handler.uuid) }</div>
                    <div class="label">{ "name: " }</div>
                    <div class="value">{ format!("{}", info_form_handler.name) }</div>
                    <div class="label">{ "group: " }</div>
                    <div class="value">{ format!("{}", info_form_handler.group) }</div>
                </div>
                <div>
                    <div class="label"><label for="props">{ "свойства виджета: " }</label></div>
                    <table class="prop-table">
                    <colgroup>
                        <col style="width: 80px"/>
                        <col style="width: 100%"/>
                    </colgroup>
                    {for widget_props_list_handler.current().iter().map(|item| {
                        html!{
                            <tr>
                                <td>{ item.name.clone() }</td>
                                <td>{ item.ds.tag.clone() }</td>
                            </tr>
                        }
                    })}
                    </table>
                </div>
                </>
            }
        };

    html! {
        <div class="item-details">
            {header}
            if *edit_mode {
                { form_view }
            } else {
                { view }
            }
        </div>
    }
}

// -----------------------------------------------
fn apply_form_to_state(
    dto: &WidgetDto,
    editor: &MxEditor,
    form_handler: UseStateHandle<WidgetForm>,
) {
    // prepare widget form
    let form = WidgetForm {
            uuid: dto.uuid.clone().into(),
            name: dto.name.clone().into(),
            group: dto.group.clone().into(),
            diagram_meta: get_cell0_meta(editor).unwrap_or(DiagramMeta::get_widget_default()),
        };

    form_handler.set(form);
}

//
// fn set_state_model_meta(dispatch: Dispatch<State>, created: WidgetDto) {
//     // set model meta
//     dispatch.reduce_mut(|state| {
//         state.model_meta = ModelForm::Widget(WidgetForm {
//             uuid: created.uuid.into(),
//             name: created.name.into(),
//             group: created.group.into(),
//             ..Default::default()
//         });
//     });
// }

// // -----------------------------------------------
// fn get_widget_form_meta(
//     editor: &MxEditor,
//     form: &WidgetForm,
//     widget_props_list_handler: UseListHandle<WidgetPropertyXml>
// ) -> WidgetForm {
//     if let Some((outer_html, meta)) =  get_cell0_meta(editor) {

//         // state widget properties
//         widget_props_list_handler.set(meta.property.clone());

//         // decorate form meta
//         let mut form = form.clone();
//         form.object_meta = outer_html;

//         return form;
//     }

//     // else
//     Default::default()
// }

// ---------------------------------------------
pub fn decorate_with_cell0_meta(
    editor:&MxEditor,
    form: &mut WidgetForm,
    list_handle: UseListHandle<WidgetPropertyXml>
) {
    if let Some(diagram_meta) = get_cell0_meta(editor) {
        // set form diagram_meta
        form.diagram_meta = diagram_meta.clone();

        // set widget properties
        if let GraphModel::Widget(widget_meta) = diagram_meta.model {
            let props_list = widget_meta.property.clone();
            list_handle.set(props_list);
        }
    }
}