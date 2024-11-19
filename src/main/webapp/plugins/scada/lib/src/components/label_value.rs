use std::{cell::RefCell, rc::Rc};

use common_model::{data_source::DataSourceXml, label_value::LabelValueXml};
use yew::{function_component, html, use_effect_with, use_state, Callback, Html, Properties };
use yew_hooks::use_unmount;
use yewdux::use_selector;

use crate::{
    components::data_source::{self, DataSource}, errors::CellStateError, model::cell_meta::{CellMeta, CellMetaVariant}, store::cell
};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub edit_mode: bool,
    pub cell_meta: Rc<RefCell<CellMeta>>,
    pub on_detals_apply: Callback<CellMetaVariant>,
}

#[function_component]
pub fn LabelValueComponent(Props {
    edit_mode, 
    cell_meta, 
    on_detals_apply,
}: &Props ) -> Html 
{
    use_unmount(|| {
        log::debug!("LabelValueComponent unmount");
    });    
    
    // let (_, store_state_dispatch) = use_store::<cell::State>();
    let label_state = use_state(|| {
            if let Ok(label) = cell_meta.borrow().get_label_meta() {
                return label;
            };
            log::warn!("{}", CellStateError::NotLabel);
            LabelValueXml::default()           
        });
    let data_source = use_state(|| label_state.ds.clone());

    let start_apply = use_selector(|state: &cell::State | state.start_apply);
    {    
        let label_state = label_state.clone();
        let on_detals_apply = on_detals_apply.clone();
        let data_source = data_source.clone();
        use_effect_with(*start_apply, move |start| {
            if *start {
                let mut new_label = (*label_state).clone();
                new_label.ds = (*data_source).clone();

                let new_variant = CellMetaVariant::Label(new_label);
                on_detals_apply.emit(new_variant);
            }
        })
    };

    // let is_edit = use_state(|| false);
    // let togle_edit = {
    //     let edit = is_edit.clone();
    //     Callback::from(move |_: MouseEvent| { edit.set(!*edit); })
    // };  

    // let label_value_apply = store_state_dispatch.apply_callback(|value: LabelValueXml| ApplyLabelValueMetaAction(value));  

    // let on_apply = {
    //         let is_edit = is_edit.clone();
    //         // let label_state = label_state.clone();
    //         // let apply = apply.clone();
    //         let label_value_apply = label_value_apply.clone();
    //         Callback::from(move |value: LabelValueXml| {
    //             label_value_apply.emit(value);
    //             is_edit.set(false);     // togle is_edit
    //         })
    //     };        

    let apply_ds = {
            let data_source = data_source.clone();
            Callback::from(move |ds: DataSourceXml| {
                data_source.set(ds);
            })
        };        

    // let on_cancel = {
    //         let is_edit = is_edit.clone();
    //         let label_state = label_state.clone();
    //         let label_original = value.clone();
    //         Callback::from(move |_: MouseEvent| {
    //             let val = label_original.clone();
    //             label_state.set(val);
    //             is_edit.set(false);     // togle is_edit
    //         })
    //     };    

    // // tag name input
    // let oninput = {
    //         let label_state = label_state.clone();
    //         Callback::from(move |e:InputEvent| {
    //             e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
    //                 .map(|input| {
    //                     let mut ds = label_state.ds.clone();
    //                     ds.tag = input.value().into();
    //                     label_state.set(LabelValueXml { ds });
    //                 });
    //         })
    //     };

    // let tag_view = {
    //     let value = label_state.clone();
    //     html! {
    //         if *(is_edit.clone()) {
    //             <input id="tag" {oninput} value={ format!("{}", value.ds.tag) }/>
    //         } else {
    //             {value.ds.tag.clone()}
    //         }
    //     }
    // };    

    let data_source_view = {
        let data_source = data_source.clone();
        let apply_ds = apply_ds.clone();
        let props = yew::props!(data_source::Props {
            ds: (*data_source).clone(),
            edit_mode: *edit_mode,
            on_apply: apply_ds,
        });
        html! {<DataSource ..props/>}
    };


    html!{
        <fieldset>
            <legend>{"Настройки значения:"}</legend>
            
            // <table class="prop-table">
            //     <td class="label" width="20">{"tag"}</td>
            //     <td>{ tag_view }</td>
            //     <td class="img">
            //         <EditButtons edit_mode={*edit_mode} is_edit={is_edit.clone()}
            //             on_apply={togle_apply.clone()}
            //             on_edit={togle_edit.clone()}
            //             on_cancel={togle_cancel.clone()}
            //         />
            //     </td>
            // </table>

           { data_source_view }
        </fieldset>
    }
    
}