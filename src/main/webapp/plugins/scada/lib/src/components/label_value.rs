use common_model::free_value::LabelValueXml;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_effect_with, use_state, Callback, Html, InputEvent, MouseEvent, Properties};
use yew_hooks::use_unmount;
use yewdux::{use_selector, use_store};

use crate::{components::shared::{MdIcon, MdIconType}, model::cell_meta::CellType, store::cell::{self, SetLabelAction}};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub edit_mode: bool,
    #[prop_or_default]
    pub value: LabelValueXml,
    #[prop_or_default]
    pub apply: Callback<LabelValueXml>,
    pub on_detals_apply: Callback<CellType>,
}

#[function_component]
pub fn LabelValueComponent(Props {edit_mode, value, apply, on_detals_apply}: &Props ) -> Html 
{
    use_unmount(|| {
        log::debug!("LabelValueComponent unmount");
    });    
    
    let (_, store_state_dispatch) = use_store::<cell::State>();

    let start_apply = use_selector(|state: &cell::State | state.start_apply);
    let label_state = use_state(|| value.clone());
    {    
        let label_state = label_state.clone();
        let on_detals_apply = on_detals_apply.clone();
        use_effect_with(*start_apply, move |start| {
            if *start {
                log::debug!("label appply");
                store_state_dispatch.apply(SetLabelAction((*label_state).clone()));
                on_detals_apply.emit(CellType::LABEL);
            }
        })
    };


    let is_edit = use_state(|| false);
    let togle_edit = {
        let edit = is_edit.clone();
        Callback::from(move |_: MouseEvent| { edit.set(!*edit); })
    };  

    let togle_apply = {
        let is_edit = is_edit.clone();
        let label_state = label_state.clone();
        let apply = apply.clone();
        Callback::from(move |_: MouseEvent| {
            apply.emit((*label_state).clone());
            is_edit.set(!*is_edit);     // togle is_edit
        })
    };        

    // tag name input
    let oninput = {
            let label_state = label_state.clone();
            Callback::from(move |e:InputEvent| {
                e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                    .map(|input| {
                        let mut ds = label_state.ds.clone();
                        ds.set_tag(input.value().into());
                        label_state.set(LabelValueXml { ds });
                    });
            })
        };

    // item view
    let img_view = {
        let is_edit = is_edit.clone();
        if *edit_mode && *is_edit { 
           html! { <button onclick={togle_apply}><MdIcon icon={MdIconType::Check}/></button> }
        } else if *edit_mode {
           html! { <button onclick={togle_edit}><MdIcon icon={MdIconType::Edit}/></button> }
        } else {
           html! {  }
        }
    };    

    let tag_view = {
        let value = label_state.clone();
        html! {
            if *(is_edit.clone()) {
                <input id="tag" {oninput} value={ format!("{}", value.ds.tag) }/>
            } else {
                {value.ds.tag.clone()}
            }
        }
    };    

    html!{
        <fieldset>
            <legend>{"Настройки значения:"}</legend>
            
            <table class="prop-table">
                <td class="label" width="20">{"tag"}</td>
                <td>{ tag_view }</td>
                <td class="img">{ img_view }</td>
            </table>
        </fieldset>
    }
    
}