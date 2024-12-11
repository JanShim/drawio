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


#[wasm_bindgen(js_name=initSchemaRender)]
pub fn init_schema_render(mx_editor: MxEditor, mx_utils: MxUtils, div: HtmlDivElement, options: SchemaOptions) {
    Dispatch::<store::diagram::State>::global().set(store::diagram::State {
        redraw: Default::default(),
        model_meta: Default::default(),
    });

    let props = Props { context: MxGraphContext {
        api_url: options.api_url.unwrap_or("undefiend".to_owned()).into(),
        mx_utils,
        mx_editor,
    }.into() };

    yew::Renderer::<DiagramApp>::with_root_and_props(div.into(), props).render();
    log::info!("schema loaded");
}

// #[wasm_bindgen(js_name=renderSchema)]
// pub fn render_schema(mx_utils: MxUtils, mx_editor: MxEditor, div: HtmlDivElement, options: SchemaOptions) {
//     // Dispatch::<store::diagram::State>::global().set(store::diagram::State {
//     //     api_url: options.api_url.unwrap_or("undefiend".to_owned()).into(),
//     //     mx_utils,
//     //     mx_editor,
//     //     model_meta: Default::default(),
//     // });

//     // yew::Renderer::<App>::with_root(div.into()).render();
//     // log::info!("schema loaded");
// }

#[wasm_bindgen(js_name=recreateModelMeta)]
pub fn recreate_model_meta(model_type: JsString) {
    let dispatch = Dispatch::<store::diagram::State>::global();
    match model_type.as_string() {
        Some(string) if string == "widget" => dispatch.reduce_mut(|state| {
            state.redraw = true;
            state.model_meta = ModelForm::Widget(Default::default())
        }),
        _ => dispatch.reduce_mut(|state| {
            state.model_meta = ModelForm::Diagram(Default::default())
        }),
    }
}
