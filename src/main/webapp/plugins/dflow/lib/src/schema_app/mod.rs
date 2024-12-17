use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yewdux::Dispatch;

use web_sys::{js_sys::JsString, HtmlDivElement};

use crate::{
    components::{get_global_css, InfoComponent},
    model::{
        common::ModelForm,
        mx_editor::MxEditor,
        mx_utils::MxUtils,
    },
    store::{self, mx_context::{MxGraphContext, TMxGraphContext}},
    utils::{SchemaOptions, NULL_UUID}
};

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    pub context: TMxGraphContext,
}


#[function_component]
pub fn DiagramApp(Props { context }: &Props) -> Html {
    // =================== view ====================
    html! {
        <>
            { get_global_css() }

            <ContextProvider<TMxGraphContext> context={context.clone()}>
                <InfoComponent />
            </ContextProvider<TMxGraphContext>>
        </>
    }
}

#[wasm_bindgen(js_name=recreateModelMeta)]
pub fn recreate_model_meta(
    model_type: JsString,
    mx_editor: MxEditor,
    mx_utils: MxUtils,
    div: HtmlDivElement,
    options: SchemaOptions
) {
    match model_type.as_string() {
        Some(string) if string == "widget" => {
            Dispatch::<store::diagram::State>::global().set(store::diagram::State {
                model_meta: ModelForm::Widget(Default::default()),
            });
        },
        _ => {
            Dispatch::<store::diagram::State>::global().set(store::diagram::State {
                model_meta: ModelForm::Diagram(Default::default()),
            });
        }
    }

    let props = Props { context: MxGraphContext {
        api_url: options.api_url.unwrap_or("undefiend".to_owned()).into(),
        mx_utils,
        mx_editor,
    }.into() };

    yew::Renderer::<DiagramApp>::with_root_and_props(div.into(), props).render();
    log::info!("schema loaded");
}
