use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::{FormData, HtmlFormElement};
use yew_hooks::{use_async_with_options, use_list, use_toggle, UseAsyncOptions, UseListHandle};
use yewdux::{use_selector, use_store, Dispatch};
use common_model::diagram::{WidgetPropertyXml, WidgetXml};

use crate::{
    components::shared::{use_list_selected, MdIcon, MdIconType}, model::{
        common::{DiagramMeta, GraphModel, ModelForm}, mx_cell::CellValue, mx_editor::MxEditor, widget::{form_meta::WidgetForm, WidgetDto}, widget_group::WidgetGroupListItemDto
    },
    store::{cell::NO_CONTEXT_FOUND, diagram::{self, State}, mx_context::TMxGraphContext},
    utils::{cliped_model_box, fetch, get_cell0, get_cell0_widget_meta, post, put, set_cell0_value}
};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub form_meta: WidgetForm,
}

#[function_component]
pub fn WidgetInfoComponent(Props{ form_meta }: &Props) -> Html {
    let mx_graph_context = use_context::<TMxGraphContext>().expect(NO_CONTEXT_FOUND);
    let (state, dispatch) = use_store::<diagram::State>();
    let redraw = use_selector(|state: &diagram::State| {state.redraw});

    let widget_props_list = use_list(Vec::<WidgetPropertyXml>::new());
    // {
    //     let editor = mx_graph_context.mx_editor.clone();
    //     let widget_props_list = widget_props_list.clone();
    //     let dispatch = dispatch.clone();
    //     use_effect_with(redraw.clone(), move |redraw| {
    //         if **redraw {
    //             let form = recreate_meta(&editor, widget_props_list);
    //             dispatch.reduce_mut(|state| {
    //                 state.redraw = false;
    //                 state.model_meta = ModelForm::Widget(form);
    //             });
    //         }
    //     })
    // }

    let widget_form = {
        let editor = &mx_graph_context.mx_editor;
        let widget_props_list = widget_props_list.clone();
        let form_meta = form_meta;
        use_state(move || {
            let form = get_widget_form_meta(editor, form_meta, widget_props_list);
            log::debug!("use_state::: {form:?}");
            form
        })
    };
    {
        let widget_form = widget_form.clone();
        use_effect_with((*form_meta).clone(), move |form| {
            log::debug!("use_effect_with::: {form:?}");
            widget_form.set((*form).clone());
        })
    }


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
    //                 //         widget_props_list.set(meta.property);       // state widget properties
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

    {
        let editor = &mx_graph_context.mx_editor;
        let widget_form = widget_form.clone();
        let widget_props_list = widget_props_list.clone();
        use_effect_with(get_cell0_widget_meta(editor), move |widget_meta| {
            if let Some((meta, widget_xml)) = widget_meta {
                log::debug!("meta::: {meta}; widget_xml:::: {:?}", widget_xml);
                let props_list = widget_xml.property.clone();
                widget_props_list.set(props_list);
                let mut form = (*widget_form).clone();
                form.meta = meta.clone();
                widget_form.set(form);
            }
        });
    }

    // let model_meta = {
    //     let mx_graph_context = mx_graph_context.clone();
    //     let widget_props_list = widget_props_list.clone();
    //     use_selector(move |state: &diagram::State| {
    //         match &state.model_meta {
    //             ModelForm::Widget(form) => {
    //                 let editor = &mx_graph_context.mx_editor;
    //                 if let Ok(CellValue::Object(el)) = get_cell0(editor).get_value() {
    //                     if let Ok(meta) = quick_xml::de::from_str::<WidgetXml>(el.inner_html().as_str()) {
    //                         // appy meta to store
    //                         let mut form = form.clone();
    //                         form.meta = el.outer_html().into();     // this is cell0 value
    //                         widget_props_list.set(meta.property);       // state widget properties
    //                         return form;
    //                     }
    //                 } else {
    //                     log::debug!("else CellValue: {:?}", get_cell0(editor).get_value().unwrap());
    //                 }
    //                 form.clone()
    //             },
    //             _ => {
    //                 log::info!("this is not widget item");
    //                 Default::default()
    //             },
    //         }
    //     })};

    let edit_mode = use_toggle(false, true);

    let url = mx_graph_context.api_url.clone();
    let widget_groups_list = use_async_with_options(
        async move { fetch::<Vec::<WidgetGroupListItemDto>>(format!("{url}/widget-group/list")).await },
        UseAsyncOptions::enable_auto(),
    );

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
        let widget_form = widget_form.clone();
        let widget_props_list = widget_props_list.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let form = event.target()
                .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

            if let Some(form) = form {
                if let Some(form) = FormData::new_with_form(&form).ok().map(|data| Into::<WidgetForm>::into(data)) {
                    let meta = form.meta.to_string();

                    log::debug!("CURR meta: {}", meta);

                    // chage cell0 in graph model
                    set_cell0_value(&mx_graph_context.mx_editor, meta.clone());

                    // ctore new property list
                    if let Ok(meta) = quick_xml::de::from_str::<DiagramMeta>(&meta) {
                        if let GraphModel::Widget(widget) = meta.model {
                            widget_props_list.set(widget.property);
                        }
                    }

                    // store form meta to state
                    widget_form.set(form.clone());

                    // send to db
                    let dispatch = dispatch.clone();
                    let mx_graph_context = mx_graph_context.clone();
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
                                    if let Ok(created) = result {
                                        set_state_model_meta(dispatch, created);
                                    } else {
                                        log::debug!("widget create error: {}", result.unwrap_err());
                                    }
                                } else {
                                    let item = WidgetDto {
                                        uuid: form.uuid.to_string(),
                                        group: form.group.to_string(),
                                        name: form.name.to_string(),
                                        model,
                                        types: vec!["ZDV2".to_owned()],
                                        svg: Some(svg),
                                    };

                                    let result = put(format!("{}/widget/{}", mx_graph_context.api_url, form.uuid), item).await;
                                    if let Ok(updated) = result {
                                        set_state_model_meta(dispatch, updated);
                                    } else {
                                        log::debug!("widget {} update error: {}", form.uuid, result.unwrap_err());
                                    }
                                }
                            };
                        }
                    });

                edit_mode.set(false);
            }
        }
    })};

    let on_property_add = {
            let widget_props_list = widget_props_list.clone();
            Callback::from(move |event: MouseEvent| {
                event.prevent_default();
                widget_props_list.push(WidgetPropertyXml::default());
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
            let widget_form = widget_form.clone();
            if widget_groups_list.loading {
                html! {  }
            } else  {
                let selected_group =  widget_form.group.clone();
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
            let widget_form = widget_form.clone();
            html! {
                <form onsubmit={on_apply}>
                    <input type="hidden" name="uuid" value={ widget_form.uuid.clone() }/>
                    <input type="hidden" name="meta" value={ widget_form.meta.clone() }/>

                    <div class="label"><label for="uuid">{ "uuid: " }</label></div>
                    <input name="uuid-0" value={ format!("{}", widget_form.uuid) } disabled={true} class="input-100"/><br/>
                    <div class="label"><label for="name">{ "name: " }</label></div>
                    <input name="name" value={ format!("{}", widget_form.name) } class="input-100"/><br/>
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
                            {for widget_props_list.current().iter().map(|item| {
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
            let widget_form = widget_form.clone();
            html! {
                <>
                <div>
                    <div class="label">{ "uuid: " }</div>
                    <div class="value">{ format!("{}", widget_form.uuid) }</div>
                    <div class="label">{ "name: " }</div>
                    <div class="value">{ format!("{}", widget_form.name) }</div>
                    <div class="label">{ "group: " }</div>
                    <div class="value">{ format!("{}", widget_form.group) }</div>
                </div>
                <div>
                    <div class="label"><label for="props">{ "свойства виджета: " }</label></div>
                    <table class="prop-table">
                    <colgroup>
                        <col style="width: 80px"/>
                        <col style="width: 100%"/>
                    </colgroup>
                    {for widget_props_list.current().iter().map(|item| {
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
        <>
            {header}
            if *edit_mode {
                { form_view }
            } else {
                { view }
            }
        </>
    }
}

// -----------------------------------------------
fn set_state_model_meta(dispatch: Dispatch<State>, created: WidgetDto) {
    // set model meta
    dispatch.reduce_mut(|state| {
        state.model_meta = ModelForm::Widget(WidgetForm {
            uuid: created.uuid.into(),
            name: created.name.into(),
            group: created.group.into(),
            ..Default::default()
        });
    });
}

// -----------------------------------------------
fn get_widget_form_meta(
    editor: &MxEditor,
    form: &WidgetForm,
    widget_props_list: UseListHandle<WidgetPropertyXml>
) -> WidgetForm {
    if let Some((outer_html, meta)) =  get_cell0_widget_meta(editor) {

        // state widget properties
        widget_props_list.set(meta.property.clone());

        // decorate form meta
        let mut form = form.clone();
        form.meta = outer_html;

        return form;
    }

    // else
    Default::default()
}