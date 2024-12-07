use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use yewdux::{use_selector, use_store};

use crate::{
    components::shared::{MdIcon, MdIconType}, model::{
        common::ModelForm,
        widget::{form_meta::WidgetForm, WidgetDto}, widget_group::WidgetGroupListItemDto
    },
    store::{cell::NO_CONTEXT_FOUND, diagram, mx_context::TMxGraphContext},
    utils::{cliped_model_box, fetch, post, put}
};

#[function_component]
pub fn WidgetInfoComponent() -> Html {
    let mx_graph_context = use_context::<TMxGraphContext>().expect(NO_CONTEXT_FOUND);
    let (state, dispatch) = use_store::<diagram::State>();
    let model_meta = use_selector(|state: &diagram::State| {
        // log::debug!("selector: {:?}", state.model_meta);
        match &state.model_meta {
            ModelForm::Widget(form) => form.clone(),
            _ => {
                log::info!("this is not widget item");
                Default::default()
            },
        }
    });

    let edit_mode = use_state(|| false);

    let url = mx_graph_context.api_url.clone();
    let widget_groups_list = use_async_with_options(
        async move { fetch::<Vec::<WidgetGroupListItemDto>>(format!("{url}/widget-group/list")).await },
        UseAsyncOptions::enable_auto(),
    );

    // ============= events ====================
    let edit_mode_toggle = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |_: MouseEvent| { edit_mode.set(true); })
    };

    let on_cancel = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |_: MouseEvent| {
            edit_mode.set(false);
        })
    };

    let on_apply = {
        let mx_graph_context = mx_graph_context.clone();
        let edit_mode = edit_mode.clone();
        // let state = state.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let form = event.target()
                .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

            if let Some(form) = form {
                if let Some(form) = FormData::new_with_form(&form).ok().map(|data| Into::<WidgetForm>::into(data)) {
                    // let state = state.clone();

                    // appy to store
                    dispatch.reduce_mut(|state| {
                        state.model_meta = ModelForm::Widget(form.clone());
                    });

                    // send to db
                    let dispatch = dispatch.clone();
                    let mx_graph_context = mx_graph_context.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Ok(node) = mx_graph_context.get_graph_xml() {
                            if let Ok(Some(model_str)) = mx_graph_context.get_xml(node) {
                                let svg = mx_graph_context.get_graph_svg();

                                if form.is_new_item() {
                                    let item = WidgetDto::new(
                                        form.group.to_string(),
                                        form.name.to_string(),
                                        cliped_model_box(model_str).into(),
                                        vec![],
                                        Some(svg)
                                    );

                                    // log::debug!("post: {item:?}");

                                    let created = post(format!("{}/widget", mx_graph_context.api_url), item).await
                                        .and_then(|dto| {
                                            // log::debug!("created: {dto:?}");
                                            Ok(dto)
                                        }).unwrap();


                                    // set model meta
                                    dispatch.reduce_mut(|state| {
                                        state.model_meta = ModelForm::Widget(WidgetForm {
                                            uuid: created.uuid.into(),
                                            name: created.name.into(),
                                            group: created.group.into(),
                                        });
                                    });

                                } else {
                                    let item = WidgetDto {
                                        uuid: form.uuid.to_string(),
                                        group: form.group.to_string(),
                                        name: form.name.to_string(),
                                        model: cliped_model_box(model_str).into(),
                                        types: vec!["ZDV2".to_owned()],
                                        svg: Some(svg),
                                    };

                                    let res = put(format!("{}/widget/{}", mx_graph_context.api_url, form.uuid), item).await
                                        .and_then(|dto| {
                                            log::debug!("saved:  {dto:?}");
                                            Ok(dto)
                                        });
                                    if res.is_err() {
                                        log::error!("{:?}", res.err());
                                    }

                                }
                            };
                        }
                    }
                );

                edit_mode.set(false);
            }
        }
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
            if widget_groups_list.loading {
                html! {  }
            } else  {
                let selected_group =  model_meta.group.clone();
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

    html! {
        <>
            {header}
            if *edit_mode {
            <form onsubmit={on_apply}>
                <input type="hidden" name="uuid" value={ format!("{}", model_meta.uuid) }/>

                <div class="label"><label for="uuid">{ "uuid: " }</label></div>
                <input name="uuid-0" value={ format!("{}", model_meta.uuid) } disabled={true} class="input-100"/><br/>
                <div class="label"><label for="name">{ "name: " }</label></div>
                <input name="name" value={ format!("{}", model_meta.name) } class="input-100"/><br/>
                <div class="label"><label for="group">{ "group: " }</label></div>

                // <input name="group" value={ format!("{}", model_meta.group) } class="input-100"/><br/>
                { wgroups_select }

                <div class="flex-box-2" >
                    <button type="button" onclick={on_cancel}>{"Cancel"}</button>
                    <button type="submit">{"Save"}</button>
                </div>
            </form>
            } else {
            <div>
                <div class="label">{ "uuid: " }</div>
                <div class="value">{ format!("{}", model_meta.uuid) }</div>
                <div class="label">{ "name: " }</div>
                <div class="value">{ format!("{}", model_meta.name) }</div>
                <div class="label">{ "group: " }</div>
                <div class="value">{ format!("{}", model_meta.group) }</div>
            </div>
            }
        </>
    }
}
