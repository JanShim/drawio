use wasm_bindgen::prelude::*;
use yew::prelude::*;
use std::rc::Rc;

use web_sys::{HtmlDivElement, HtmlElement};
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use stylist::yew::{styled_component, Global};

use crate::{
    errors::FetchError, 
    components::diagram::list_item::DiagramListItemComponent, 
    components::widget::list_item::WidgetListItemComponent, 
    model::{
        mx_editor::MxEditor, 
        mx_utils::MxUtils, 
        scada_diagram::{
            DiagramListItem, 
            ScadaDiagramDto
        },
        widget::{
            WidgetListItem, 
            // ScadaDiagramDto
        }         
    }, 
    utils::{fetch, fetch_string, load_scada_model, post, SchemaOptions} 
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub api_url: String,
    pub mx_utils: Rc<MxUtils>,
    pub mx_editor: Rc<MxEditor>,
}

#[styled_component(App)]
pub fn app(Props {api_url, mx_utils, mx_editor}: &Props) -> Html {
    let editor = mx_editor.clone();

    let tab_tag = use_state(|| "diagram".to_owned());


    let url = api_url.clone();
    let diagram_list = use_async_with_options(
        async move { fetch::<Vec::<DiagramListItem>>(format!("{url}/diagram/all")).await },
        UseAsyncOptions::enable_auto(),
    );

    let url = api_url.clone();
    let widget_list = use_async_with_options(
        async move { fetch::<Vec::<WidgetListItem>>(format!("{url}/widget/all")).await },
        UseAsyncOptions::enable_auto(),
    );
    // ---------------
    // load model from db
    let on_load_model =  {
        let editor = mx_editor.clone();
        let url = api_url.clone();
        Callback::from(move |pk: String|  {
            let editor = editor.clone();
            let url = url.clone();
            wasm_bindgen_futures::spawn_local(async move {
                fetch_string(format!("{url}/diagram/{pk}/model")).await  
                .map(|model| {
                    load_scada_model(&editor, model.as_str());
                }).unwrap();
            });
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


    // ---------------
    // insert model to db
    let on_create_model =  {
        let editor = mx_editor.clone();
        let utils = mx_utils.clone();
        let url = api_url.clone();
        Callback::from(move |_: MouseEvent|  {
            let editor = editor.clone();
            let utils = utils.clone();
            let url = url.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(node) = editor.get_graph_xml() {
                    if let Ok(Some(model_str)) = utils.get_xml(node) {
                        let item = ScadaDiagramDto::new("insert proba".to_owned(), model_str);
                        post(format!("{url}/diagram"), item).await
                            .and_then(|o| Ok(o.uuid))
                            .map(|pk| {
                                wasm_bindgen_futures::spawn_local(async move {
                                    fetch_string(format!("{url}/diagram/{pk}/model")).await
                                        .map(|model| {
                                            load_scada_model(&editor, model.as_str());
                                        }).unwrap();
                                })
                            })
                            .unwrap();
                    } 
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
                        html!{ <DiagramListItemComponent item={item.clone()} load={on_load_model.clone()}/> }
                    )
            })      
        }    
    };

    let widgets_view = {
        if widget_list.loading {
            html! { "Loading, wait a sec..." }
        } else  {
            widget_list.data.as_ref().map_or_else(
                || html! {},        // default
                |repo| html! { 
                    for repo.iter().map(|item| 
                        html!{ <WidgetListItemComponent item={item.clone()} load={on_load_model.clone()}/> }
                    )
            })      
        }   
    };

    let tab_content_view = {
        match tab_tag {
            val if *val == "widgets" => widgets_view,
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
        
        "#)} />

        <div class="tab">
          <button tag="diagrams" class="tablinks active" onclick={on_tab_select.clone()}>{"Diagrams"}</button>
          <button tag="widgets" class="tablinks" onclick={on_tab_select.clone()}>{"Widgets"}</button>
        </div>
        
        <div class="tabcontent">
            { tab_content_view }
        </div>
        
<hr/>
            // <p>{api_url}</p>

           
                   
            <p>{
                diagram_list.error.as_ref().map_or_else(|| html! {}, |error| match error {
                    FetchError::SerdeError(err) => html! { err },
                    FetchError::RequestError(err) => html! { err },
                    FetchError::InsertModelError(err) => html!{ err },
                    FetchError::ParseXmlError(err) => html!{ err },
                })
            }</p>

        </>
    }    
}


#[wasm_bindgen(js_name=openDialog)]
pub fn open_dialog(mx_utils: MxUtils, mx_editor: MxEditor, div: HtmlDivElement, options: SchemaOptions) {
    let props  = Props {
        api_url: options.api_url.unwrap_or("undefiend".to_owned()),
        mx_utils: Rc::new(mx_utils),
        mx_editor: Rc::new(mx_editor),
    };
    yew::Renderer::<App>::with_root_and_props(div.into(), props).render();
    log::info!("schema loaded");
}




