use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_effect_with, use_memo, use_state, Callback, Html, InputEvent, MouseEvent, Properties };
use common_model::data_source::{self, DataSourceXml};

use crate::components::shared::{use_state_with, EditButtons};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub ds: DataSourceXml,
    pub edit_mode: bool,
    pub on_apply: Callback<DataSourceXml>,
}

#[function_component]
pub fn DataSource(Props {ds, edit_mode, on_apply}: &Props ) -> Html 
{
    let data_source = use_state_with(ds.clone());

    let ds_original = use_memo(ds.clone(), |ds| ds.clone());

    let is_edit = use_state(|| false);

    let on_edit = {
        let is_edit = is_edit.clone();
        Callback::from(move |_: MouseEvent| { is_edit.set(true); })
    };  

    let on_cancel = {
            let is_edit = is_edit.clone();
            let data_source = data_source.clone();
            Callback::from(move |_: MouseEvent| {
                let val = (*ds_original).clone();
                data_source.set(val);
                is_edit.set(false);     // togle is_edit
            })
        };

    let on_apply = {
            let is_edit = is_edit.clone();
            let ds = data_source.clone();
            let on_apply = on_apply.clone();
            Callback::from(move |_: MouseEvent| {
                on_apply.emit((*ds).clone());
                is_edit.set(!*is_edit);     // togle is_edit
            })
        };        

    // tag name input
    let on_tag_input = {
            let ds = data_source.clone();
            Callback::from(move |e:InputEvent| {
                e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                    .map(|input| {
                        let mut val = (*ds).clone();
                        val.tag = input.value().into();
                        ds.set(val);
                    });
            })
        };
  
    let tag_view = {
        let ds = data_source.clone();
        let is_edit = is_edit.clone();
        html! {
            if *edit_mode && *is_edit {
                <input id="tag" oninput={on_tag_input} value={ format!("{}", ds.tag) }/>
            } else {
                {ds.tag.clone()}
            }
        }
    };    

    html!{
        <table class="prop-table">
        <tr>
            <td class="label" width="20">{"Тэг"}</td>
            <td>{ tag_view }</td>
            <td class="img">
                <EditButtons edit_mode={*edit_mode} 
                    {is_edit}
                    {on_apply}
                    {on_edit}
                    {on_cancel}
                />
            </td>
        </tr>
        </table>
    }
    
}

