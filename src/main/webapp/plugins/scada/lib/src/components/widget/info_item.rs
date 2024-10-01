use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};
use yew::{
    function_component, html, use_state, Callback, Html, MouseEvent, Properties, SubmitEvent
};
use yewdux::{use_selector, use_store};

use crate::{model::widget::{meta::Widget, WidgetDto}, store::diagram, utils::{fetch_string, load_scada_model, post}};


#[derive(PartialEq, Properties)]
pub struct Props {
    pub widget: Widget,
}

#[function_component(WidgetInfoComponent)]
pub fn scada_diagram_component(Props { widget }: &Props) -> Html {
    let (diagram_state, _) = use_store::<diagram::State>();
    let api_url = use_selector(|state: &diagram::State| { state.api_url.clone() });
    let mx_utils = use_selector(|state: &diagram::State| { state.mx_utils.clone().unwrap() });
    let mx_editor = use_selector(|state: &diagram::State| { state.mx_editor.clone().unwrap() });

    let edit_mode = use_state(|| false);

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
        let edit_mode = edit_mode.clone();
        let editor = mx_editor.clone();
        let utils = mx_utils.clone();
        let url = api_url.clone();        
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let form = event.target()
                .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

            if let Some(form) = form {
                if let Some(widget) = FormData::new_with_form(&form).ok().map(|data| Into::<Widget>::into(data)) {
                    // lets create widget in db
                    let editor = editor.clone();
                    let utils = utils.clone();
                    let url = url.clone();        
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Ok(node) = editor.get_graph_xml() {
                            if let Ok(Some(model_str)) = utils.get_xml(node) {
                                let item = WidgetDto::new(
                                    widget.group.to_string(), 
                                    widget.name.to_string(),
                                    model_str,
                                    vec![]
                                ); 
                                post(format!("{url}/widget"), item).await
                                    .and_then(|o| Ok(o.uuid))
                                    .map(|pk| {
                                        wasm_bindgen_futures::spawn_local(async move {
                                            fetch_string(format!("{url}/widget/{pk}/model")).await
                                                .map(|model| {


                                                    load_scada_model(&editor, model.as_str());
                                                }).unwrap();
                                        })
                                    })
                                    .unwrap();
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
            <button onclick={edit_mode_toggle}><img src="images/edit16.png"/></button>
        }
        </div>           
    };
        
    html! {
        <>
            {header}
            if *edit_mode {
            <form onsubmit={on_apply}>
                <input type="hidden" name="uuid" value={ format!("{}", widget.uuid) }/>
                <label for="uuid">{ "uuid: " }</label>
                <input name="uuid-0" value={ format!("{}", widget.uuid) } disabled={true} class="input-100"/><br/>
                <label for="name">{ "name: " }</label>
                <input name="name" value={ format!("{}", widget.name) } class="input-100"/><br/>
                <label for="group">{ "group: " }</label>
                <input name="group" value={ format!("{}", widget.group) } class="input-100"/><br/>

                <div class="flex-box-2" >
                    <button type="button" onclick={on_cancel}>{"Cancel"}</button>
                    <button type="submit">{"Save"}</button>
                </div>
            </form>
            } else {
            <div>
                { "uuid: " }<br/>
                { format!("{}", widget.uuid) }<br/>
                { "name: " }<br/>
                { format!("{}", widget.name) }<br/>
                { "group: " }<br/>
                { format!("{}", widget.group) }<br/>                
            </div>    
            }
        </>
    }
}

