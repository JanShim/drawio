
use yew::prelude::*;
use yew_hooks::{use_toggle, use_unmount};
use yewdux::{use_selector, use_store};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};

use crate::{components::shared::{MdIcon, MdIconType},
    model::{
        common::{GraphModel, ModelForm}, diagram::{form::DiagramForm, DiagramDto}, mx_editor::MxEditor
    },
    store::{cell::NO_CONTEXT_FOUND, mx_context::TMxGraphContext}, utils::{get_cell0_meta, set_cell0_value}
};
use crate::store;
use crate::utils::{post, put};


#[derive(PartialEq, Properties)]
pub struct Props {
    pub form: DiagramForm,
}

#[function_component]
pub fn DiagramInfoComponent(Props { form }: &Props) -> Html {
    let mx_graph_context = use_context::<TMxGraphContext>().expect(NO_CONTEXT_FOUND);
    use_unmount(|| { log::debug!("unmount DiagramInfoComponent") });

    // // let (state, dispatch) = use_store::<store::diagram::State>();
    // let info_meta = use_selector(|state: &store::diagram::State| {
    //     // log::debug!("selector: {:?}", state.model_meta);
    //     match &state.model_meta {
    //         ModelForm::Diagram(form) => form.clone(),
    //         _ => {
    //             log::info!("this is not diagram item");
    //             Default::default()
    //         },
    //     }
    // });

    let info_form = {
        let editor = &mx_graph_context.mx_editor;
        let mut form = form.clone();
        use_state(move || {
            // decorate meta from cell0
            decorate_with_cell0_meta(&editor, &mut form);

            log::debug!("from in use_state:: {form:?}");

            //result
            form
        })};

    let edit_mode = use_toggle(false, true);

    // ============= events ==========================
    let edit_mode_toggle = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |_| { edit_mode.toggle(); })
    };

    let on_cancel = {
        // let state = state.clone();

        let edit_mode = edit_mode.clone();
        Callback::from(move |_: MouseEvent| {

            // let bounding_box = state.get_diagram_bounding_box().unwrap();
            // log::debug!("get_diagram_bounding_box : {bounding_box:?}");

            edit_mode.set(false);
        })
    };

    let on_apply = {
        let mx_graph_context = mx_graph_context.clone();
        let edit_mode = edit_mode.clone();
        let info_form = info_form.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let form = event.target()
                .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

            if let Some(form) = form {
                if let Some(form) = FormData::new_with_form(&form).ok().map(|data| Into::<DiagramForm>::into(data)) {
                    // let state = state.clone();

                    // // appy to store
                    // dispatch.reduce_mut(|state| {
                    //     state.model_meta = ModelForm::Diagram(form.clone());
                    // });

                    // // send to db
                    // let dispatch = dispatch.clone();
                    // let mx_graph_context = mx_graph_context.clone();
                    // wasm_bindgen_futures::spawn_local(async move {

                    //     log::debug!("mx_graph_context {:?}", mx_graph_context);

                    //     if let Ok(node) = mx_graph_context.get_graph_xml() {
                    //         if let Ok(Some(model_str)) = mx_graph_context.get_xml(node) {

                    //             // log::debug!("saving: {model_str}");

                    //             let svg = mx_graph_context.get_graph_svg();

                    //             if form.is_new_item() {
                    //                 let item = DiagramDto::new(
                    //                     form.name.to_string(),
                    //                     model_str,
                    //                     Some(svg),
                    //                 );

                    //                 let created = post(format!("{}/diagram", mx_graph_context.api_url), item).await
                    //                     .and_then(|dto| {
                    //                         // log::debug!("created: {dto:?}");
                    //                         Ok(dto)
                    //                     }).unwrap();

                    //                 // set model meta
                    //                 dispatch.reduce_mut(|state| {
                    //                     state.model_meta = ModelForm::Diagram(DiagramForm {
                    //                         uuid: created.uuid.into(),
                    //                         name: created.name.into(),
                    //                     });
                    //                 });

                    //             } else {
                    //                 let item = DiagramDto {
                    //                     uuid: form.uuid.to_string(),
                    //                     name: form.name.to_string(),
                    //                     model: model_str,
                    //                     svg: Some(svg),
                    //                 };

                    //                 let res = put(format!("{}/diagram/{}", mx_graph_context.api_url, form.uuid), item).await
                    //                     .and_then(|dto| {
                    //                         log::debug!("saved:  {dto:?}");
                    //                         Ok(dto)
                    //                     });

                    //                 if res.is_err() {
                    //                     log::error!("{:?}", res.err().unwrap())
                    //                 }
                    //             }
                    //         };
                    //     }
                    // }
                    // );

                    let diagram_meta = &form.diagram_meta;

                    log::debug!("CURR meta: {:?}", diagram_meta);

                    // chage cell0 in graph model
                    let diagram_meta_str = quick_xml::se::to_string(diagram_meta).unwrap();
                    set_cell0_value(&mx_graph_context.mx_editor, diagram_meta_str);


                    // store form meta to state
                    info_form.set(form);


                    edit_mode.set(false);
                }
            }
        })};

    // ================== views ==========================
    let header = html!{
        <div class="flex-box-2 delim-label" >
        if !*edit_mode {
            <button onclick={edit_mode_toggle}><MdIcon icon={MdIconType::Edit}/></button>
        }
        </div>
    };

    let form_view = {
        let info_form = info_form.clone();
        let diagram_meta = quick_xml::se::to_string(&info_form.diagram_meta).unwrap_or_default();
        html! {
            <form onsubmit={on_apply}>
                <input type="hidden" name="uuid" value={ info_form.uuid.clone() }/>
                <input type="hidden" name="meta" value={ diagram_meta.clone() }/>

                <div class="label"><label for="uuid">{ "uuid: " }</label></div>
                <input name="uuid-0" value={ info_form.uuid.clone() } disabled={true} class="input-100"/><br/>
                <div class="label"><label for="name">{ "name: " }</label></div>
                <input name="name" value={ info_form.name.clone() } class="input-100"/><br/>

                <div class="flex-box-2" >
                    <button type="button" onclick={on_cancel}>{"Cancel"}</button>
                    <button type="submit">{"Save"}</button>
                </div>
            </form>
        }
    };

    let view = {
        let info_form = info_form.clone();
        html! {
            <div>
                <div class="label">{ "uuid: " }</div>
                <div class="value">{ info_form.uuid.clone() }</div>
                <div class="label">{ "name: " }</div>
                <div class="value">{ info_form.name.clone() }</div>
            </div>
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


// ---------------------------------------------
fn decorate_with_cell0_meta(
    editor:&MxEditor,
    form: &mut DiagramForm,
) {
    if let Some(diagram_meta) = get_cell0_meta(editor) {
        // set form diagram_meta
        form.diagram_meta = diagram_meta.clone();
    }
}