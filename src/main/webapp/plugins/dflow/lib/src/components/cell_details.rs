use yew::prelude::*;
use yew_hooks::{use_list, use_toggle, use_unmount};
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};
use common_model::dflow_cell::{CellType, DFlowVariant};

use crate::{
    components::{
        geom_value::GeomValue, label_value::LabelValueComponent,
        multystate::MultystateComponent,
        shared::{MdIcon, MdIconType},
        widget::{
            container_edit::WidgetContainerEdit,
            container_view::WidgetContainerView
        },
    },
    model::cell_meta::{ form::CellDetailsForm, TypesItem, CELL_TYPE_WIDGET_CONTAINER},
    store::cell::{
        CellInfoContext,
        NO_CONTEXT_FOUND
    }, utils::set_widget_container_glyph,
};

#[function_component]
pub fn CellDetails() -> Html {
    use_unmount(|| {
        log::debug!("CellDetailsComponent unmount");
    });

    let context = &use_context::<CellInfoContext>().expect(NO_CONTEXT_FOUND);

    let cell_types_list = use_list({
            let cell_meta = context.mx_cell.get_meta().unwrap_or_default();

            let cell_types = &cell_meta.types.iter()
                .map(|o| (o.get_cell_type(), o.clone()))
                .collect::<HashMap<CellType, DFlowVariant>>();

            if cell_types.contains_key(&CellType::WIDGETCONTAINER) {
                let type_item = TypesItem {
                        cell_type: CellType::WIDGETCONTAINER,
                        name: CELL_TYPE_WIDGET_CONTAINER.into(),
                        label: "Виджет контейнер".into(),
                        selected: true,
                    };

                let variant = cell_types.get(&CellType::WIDGETCONTAINER)
                    .map(|o| o.clone());

                // result
                vec![(type_item, variant)]
            } else {
                // result
                context.available_types.iter()
                .map(|o: TypesItem| (o.clone(), cell_types.get(&o.cell_type).map(|i| i.clone())))
                .collect::<Vec<_>>()
            }

        });

    let edit_mode = use_toggle(false, true);
    let on_edit_mode_set = {
            let edit_mode = edit_mode.clone();
            Callback::from(move |_: MouseEvent| {
                edit_mode.set(true);
            })
        };

    let on_cancel = {
            let edit_mode = edit_mode.clone();
            // let force_updater = force_updater.clone();
            Callback::from(move |_: MouseEvent| {
                edit_mode.set(false);
                // force_updater.force_update();
            })
        };

    let on_apply = {
        let context = context.clone();
        let edit_mode = edit_mode.clone();
        // let force_updater = force_updater.clone();
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
                    let mut meta = context.mx_cell.get_meta().unwrap_or_default();
                    meta.types = form.variants;

                    // apply meta to cell
                    let tst = context.mx_cell.set_meta(&meta);
                    if tst.is_err() {
                        log::error!("{:?}", tst.unwrap_err());
                    }

                    // if this is widget container. we need to update widget container model
                    if meta.types.len() == 1 && meta.types[0].get_cell_type() == CellType::WIDGETCONTAINER {
                        let model = form.widget_model.expect("model must be not empty");

                        set_widget_container_glyph(&context.mx_editor, &context.mx_cell, model.to_string());
                    }

                    // close edit mode
                    edit_mode.toggle();
                }
            }
        })};

    // ============= views ================
    let header = {
            let header_props = yew::props! { CellDetailsHeaderProps {
                edit_mode: *edit_mode,
                on_edit_mode_set,
                on_cancel,
            } };

            html! {
                <CellDetailsHeader ..header_props />
            }
        };

    let details_view = {
            let edit_mode = edit_mode.clone();
                if *edit_mode {
                    html! {
                        {for cell_types_list.current().iter()
                            .map(|(item, variant)| {
                                match item.cell_type {
                                    CellType::LABEL => {
                                        let value = variant.clone()
                                            .map(|o| o.get_label());

                                        html!{ <LabelValueComponent edit_mode={ true } {value}/> }
                                    },
                                    CellType::MULTYSTATE => {
                                        let value = variant.as_ref().map(|o| o.get_multystate());

                                        html!{ <MultystateComponent edit_mode={ true } {value}/> }
                                    },
                                    CellType::GEOM => {
                                        let value = variant.as_ref().map(|o| o.get_geometry());

                                        html!{ <GeomValue edit_mode={ true } {value}/> }
                                    },
                                    CellType::WIDGETCONTAINER => {
                                        let value = variant.as_ref()
                                            .map(|o| o.get_widget_container())
                                            .unwrap_or_default();

                                        html!{ <WidgetContainerEdit {value}/> }
                                    },
                                    _ => html!(),
                                }
                            })
                        }
                    }
                } else {
                    let current_variants = cell_types_list.current().iter()
                        .filter(|(_, variant)| variant.is_some())
                        .map(|(_, variant)| variant.clone().unwrap())
                        .collect::<Vec<_>>();

                    html! {<>
                        if current_variants.len() == 0 {
                            <h3>
                                { "Тип элемента не определен." }<br/>
                                { "Перейдите в режим редактирования." }
                            </h3>
                            //TODO: здесь можно попросить сбросить dflow
                        }
                        {for current_variants.into_iter()
                            .map(|variant| {
                                match variant {
                                    DFlowVariant::Label(value) => html!{
                                        <LabelValueComponent edit_mode={ false } {value}/>
                                    },
                                    DFlowVariant::Multystate(value) => html!{
                                        <MultystateComponent edit_mode={ false } {value}/>
                                    },
                                    DFlowVariant::Geometry(value) => html!{
                                        <GeomValue edit_mode={ false } {value}/>
                                    },
                                    DFlowVariant::WidgetContainer(value) => html!{
                                        <WidgetContainerView {value}/>
                                    },
                                    _ => html! { <h1>{ "Тип элемента не обрабатывается" }</h1> }
                                }
                            })
                        }
                    </>}
                }
        };

    html! {
        <div class="item-details">
            <form onsubmit={on_apply}>
                { header }
                { details_view }
            </form>
        </div>
    }

}

// ----------------------------------------------
#[derive(Properties, PartialEq, Debug)]
pub struct CellDetailsHeaderProps {
    pub edit_mode: bool,
    pub on_edit_mode_set: Callback<MouseEvent>,
    pub on_cancel: Callback<MouseEvent>,
}

#[function_component]
pub fn CellDetailsHeader(CellDetailsHeaderProps {
    edit_mode,
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
