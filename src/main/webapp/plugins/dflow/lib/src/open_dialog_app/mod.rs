use std::rc::Rc;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yewdux::use_store;
use web_sys::{HtmlDivElement, HtmlElement};
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use stylist::yew::{styled_component, Global};

use crate::{
    components::{
        diagram::list_item::DiagramListItemComponent,
        widget::list_item::WidgetListItemComponent
    }, model::{
        common::{DiagramMeta, GraphModel, ModelForm}, diagram::{form::DiagramForm, DiagramListItem}, editor_ui::EditorUi, mx_cell::CellValue, mx_editor::MxEditor, mx_utils::MxUtils, widget::{form::WidgetForm, WidgetListItem, WidgetProperty}
    }, schema_app::recreate_model_meta, store::diagram, utils::{fetch, fetch_string, get_cell0, get_cell0_meta, load_dflow_model, recreate_diagram_model_info, recreate_widget_model_info, SchemaOptions}
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub api_url: AttrValue,
    pub mx_utils: Rc<MxUtils>,
    pub mx_editor: Rc<MxEditor>,
    pub editor_ui: Rc<EditorUi>,
}

#[styled_component]
pub fn App(Props {api_url, mx_utils, mx_editor, editor_ui}: &Props) -> Html {
    // let editor = mx_editor.clone();
    let (_diagram_state, diagram_dispatch) = use_store::<diagram::State>();

    let tab_tag = use_state(|| "diagram".to_owned());

    let diagram_list = {
        let url = api_url.clone();
        use_async_with_options(
            async move { fetch::<Vec::<DiagramListItem>>(format!("{url}/diagram/all")).await },
            UseAsyncOptions::enable_auto(),
        )};

    let widget_list = {
        let url = api_url.clone();
        use_async_with_options(
            async move {
                fetch::<Vec::<WidgetListItem>>(format!("{url}/widget/all")).await
                    .map(|mut v| {
                        v.sort_by(|a, b| a.name.cmp(&b.name));
                        v
                    })
             },
            UseAsyncOptions::enable_auto(),
        )};

    let selected = use_state(|| AttrValue::from("undefiend"));

    // ---------------
    // load model from db
    let on_select =  {
        let selected = selected.clone();
        Callback::from(move |pk: AttrValue|  {
            log::debug!("selected: {pk:?}");
            selected.set(pk);

            // let editor = editor.clone();
            // let url = url.clone();
            // wasm_bindgen_futures::spawn_local(async move {
            //     fetch_string(format!("{url}/diagram/{pk}/model")).await
            //     .map(|model| {
            //         load_dflow_model(&editor, model.as_str());
            //     }).unwrap();
            // });
        })
    };

    let on_tab_select = {
        let tab_tag = tab_tag.clone();
        Callback::from(move |e: MouseEvent| {
            e.target().and_then(|t| t.dyn_into::<HtmlElement>().ok())
            .map(|input| {
                let val = input.get_attribute("tag").unwrap_or("diagram".to_owned());
                tab_tag.set(val);
            });
        })
    };


    // // ---------------
    // // insert model to db
    // let on_create_model =  {
    //     let editor = mx_editor.clone();
    //     let utils = mx_utils.clone();
    //     let url = api_url.clone();
    //     Callback::from(move |_: MouseEvent|  {
    //         let editor = editor.clone();
    //         let utils = utils.clone();
    //         let url = url.clone();
    //         wasm_bindgen_futures::spawn_local(async move {
    //             if let Ok(node) = editor.get_graph_xml() {
    //                 if let Ok(Some(model_str)) = utils.get_xml(node) {
    //                     let item = DFlowDiagramDto::new("insert proba".to_owned(), model_str);
    //                     post(format!("{url}/diagram"), item).await
    //                         .and_then(|o| Ok(o.uuid))
    //                         .map(|pk| {
    //                             wasm_bindgen_futures::spawn_local(async move {
    //                                 fetch_string(format!("{url}/diagram/{pk}/model")).await
    //                                     .map(|model| {
    //                                         load_dflow_model(&editor, model.as_str());
    //                                     }).unwrap();
    //                             })
    //                         })
    //                         .unwrap();
    //                 }
    //             }
    //         });
    //     })
    // };

    let on_cancel = {
        let editor_ui = editor_ui.clone();
        Callback::from(move |_: MouseEvent| {
            editor_ui.hide_dialog();
        })
    };

    let on_open = {
        let url = api_url.clone();
        let tab_tag = tab_tag.clone();
        let selected = selected.clone();
        let editor = mx_editor.clone();
        let editor_ui = editor_ui.clone();
        let mx_utils = mx_utils.clone();
        let dispatch = diagram_dispatch.clone();
        Callback::from(move |_: MouseEvent| {
            let url = url.clone();
            let tab_tag = tab_tag.clone();
            let selected = selected.clone();
            let dispatch = dispatch.clone();
            let editor = editor.clone();
            let editor_ui = editor_ui.clone();
            let mx_utils = mx_utils.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match fetch_string(format!("{url}/{}/{}/model", *tab_tag, *selected)).await {
                    Ok(model) => {
                        let meta_req = format!("{url}/{}/{}", *tab_tag, *selected);
                        if *tab_tag == "widget" {
                            match fetch::<WidgetListItem>(meta_req).await {
                                Ok(WidgetListItem { uuid, name, name_ru, group }) => {
                                    let cl_editor = editor.clone();
                                    let cb = Closure::new(move |el: JsValue| {
                                            let schema_root_container = el.dyn_into::<HtmlDivElement>().unwrap();
                                            recreate_model_meta(
                                                "widget".into(),
                                                (*cl_editor).clone(),
                                                (*mx_utils).clone(),
                                                schema_root_container,
                                                SchemaOptions { api_url: Some(url.to_string()) }
                                            );
                                        });

                                    //create new widget model
                                    recreate_widget_model_info(&editor, model, &cb);

                                    let diagram_meta = get_cell0_meta(&editor)
                                        .unwrap_or(DiagramMeta::get_widget_default());

                                    dispatch.reduce_mut(move |state| {
                                        let form = WidgetForm {
                                            uuid,
                                            name,
                                            name_ru: name_ru.unwrap_or_default(),
                                            group,
                                            diagram_meta};
                                        state.model_meta = ModelForm::Widget(form);
                                    });
                                },
                                Err(err) => log::error!("{err}"),
                            }
                        } else {
                            match fetch::<DiagramListItem>(meta_req).await {
                                Ok(DiagramListItem { uuid, name }) => {
                                    let cl_editor = editor.clone();
                                    let cb = Closure::new(move |el: JsValue| {
                                            let schema_root_container = el.dyn_into::<HtmlDivElement>().unwrap();
                                            recreate_model_meta(
                                                "diagram".into(),
                                                (*cl_editor).clone(),
                                                (*mx_utils).clone(),
                                                schema_root_container,
                                                SchemaOptions { api_url: Some(url.to_string()) }
                                            );
                                        });

                                    // create new diagram model
                                    recreate_diagram_model_info( &editor, model, &cb );

                                    let diagram_meta = get_cell0_meta(&editor)
                                        .unwrap_or(DiagramMeta::get_diagram_default());

                                    dispatch.reduce_mut(move |state| {
                                        let form = DiagramForm {uuid, name, diagram_meta};
                                        state.model_meta = ModelForm::Diagram(form);
                                    });
                                },
                                Err(err) => log::error!("{err}"),
                            }
                        }

                        // close dialog window
                        editor_ui.hide_dialog();
                    },
                    Err(err) => log::error!("{err}"),
                }
            });
        })
    };


    // ============ views ===============
    let diagrams_view = {
        if diagram_list.loading {
            html! { "Loading, wait a sec..." }
        } else  {
            diagram_list.data.as_ref().map_or_else(
                || html! {},        // default
                |repo| html! {
                    for repo.iter().map(|item|
                        html!{ <DiagramListItemComponent
                            item={item.clone()}
                            select={on_select.clone()}
                            selected={(*selected).clone()}/>
                        }
                    )
            })
        }
    };

    let widgets_view = {
        if widget_list.loading {
            html! { <tr><td>{ "Loading, wait a sec..." }</td></tr> }
        } else  {
            widget_list.data.as_ref().map_or_else(
                || html! {},        // default
                |repo| html! {
                    for repo.iter().map(|item|
                        html!{
                            <WidgetListItemComponent
                                item={item.clone()}
                                select={on_select.clone()}
                                selected={(*selected).clone()}
                            />
                        }
                    )
            })
        }
    };

    let tab_content_view = {
        let tab_tag = tab_tag.clone();
        match tab_tag {
            val if *val == "widget" => html! {
                <table>{ widgets_view }</table>
            },
            _ => diagrams_view,
        }
    };

    html! {
        <>
        <Global css={css!(r#"
/* Style the tab */
.tab {
  overflow: hidden;
  border: 1px solid #ccc;
  background-color: #f1f1f1;
}

/* Style the buttons that are used to open the tab content */
.tab button {
  background-color: inherit;
  float: left;
  border: none;
  outline: none;
  cursor: pointer;
  padding: 5px 6px;
  transition: 0.3s;
}

/* Change background color of buttons on hover */
.tab button:hover {
  background-color: #ddd;
}

/* Create an active/current tablink class */
.tab button.active {
  background-color: #ccc;
}

/* Style the tab content */
.tabcontent {
  padding: 6px 12px;
  border: 1px solid #ccc;
  border-top: none;
}

.selectable {
    cursor: pointer;
}

.selected {
    background-color:rgb(79, 146, 254);
    color: white;
}

"#)} />
        <div style="height: 340px; overflow: auto;">
            <div class="tab">
            <button tag="diagram"
                class={classes!("tablinks", (*tab_tag == "diagram").then(||Some("active")))}
                onclick={on_tab_select.clone()}>{"Diagrams"}</button>
            <button tag="widget"
                class={classes!("tablinks", (*tab_tag == "widget").then(||Some("active")))}
                onclick={on_tab_select.clone()}>{"Widgets"}</button>
            </div>

            <div class="tabcontent">
                { tab_content_view }
            </div>
        </div>
        <hr/>
        // <p>{
        //     diagram_list.error.as_ref().map_or_else(|| html! {}, |error| match error {
        //         FetchError::SerdeError(err) => html! { err },
        //         FetchError::RequestError(err) => html! { err },
        //         FetchError::InsertModelError(err) => html!{ err },
        //         FetchError::ParseXmlError(err) => html!{ err },
        //     })
        // }</p>
        <div style="margin-top: 14px; text-align: right;">
            <button class="geBtn" onclick={on_cancel}>{"Cancel"}</button>
            <button class="geBtn gePrimaryBtn" onclick={on_open}>{"Open"}</button>
        </div>

        </>
    }
}


#[wasm_bindgen(js_name=openDialog)]
pub fn open_dialog(mx_utils: MxUtils, editor_ui: EditorUi, mx_editor: MxEditor, div: HtmlDivElement, options: SchemaOptions) {
    let props  = Props {

        api_url: options.api_url.unwrap_or("undefiend".to_owned()).into(),
        mx_utils: Rc::new(mx_utils),
        mx_editor: Rc::new(mx_editor),
        editor_ui: Rc::new(editor_ui),
    };
    yew::Renderer::<App>::with_root_and_props(div.into(), props).render();
    log::info!("schema loaded");
}
