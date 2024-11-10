use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent, MouseEvent, Properties};
use yewdux::{use_store, Reducer};
use common_model::data_source::DataSourceXml;

use crate::{components::shared::{MdIcon, MdIconType}, store::cell};

// pub struct MultystateApplyDsAction(pub DataSourceXml);
// impl Reducer<cell::State> for MultystateApplyDsAction {
//     fn apply(self, state: Rc<cell::State>) -> Rc<cell::State> {
//         let mut meta= state.meta.clone();
//         if let Ok(mut multystate) = meta.get_multystate_meta() {
//             multystate.set_data_source(self.0);
//             meta.set_multystate_meta(multystate);

//             return  cell::State {
//                 meta,
//                 ..(*state).clone()
//             }.into();
//         };
//         state
//     }
// }

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub ds: DataSourceXml,
    pub edit_mode: bool,
    pub apply: Callback<DataSourceXml>,
}

#[function_component]
pub fn DataSourceComponent(Props {ds, edit_mode, apply}: &Props ) -> Html {
    // let (_, cell_store_dispatch) = use_store::<cell::State>();

    let data_source = use_state(|| ds.clone());

    let is_edit = use_state(|| false);
    let togle_edit = {
        let is_edit = is_edit.clone();
        Callback::from(move |_: MouseEvent| { is_edit.set(!*is_edit); })
    };  

    let togle_apply = {
        let is_edit = is_edit.clone();
        let ds = data_source.clone();
        let apply = apply.clone();
        Callback::from(move |_: MouseEvent| {
            // cell_store_dispatch.apply(MultystateApplyDsAction((*ds).clone()));
            apply.emit((*ds).clone());
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

    // item view
    let img_view = {
        let is_edit = is_edit.clone();
        if *edit_mode {
            if *is_edit { 
                html! { <button onclick={togle_apply}><MdIcon icon={MdIconType::Check}/></button> }
             } else {
                html! { <button onclick={togle_edit}><MdIcon icon={MdIconType::Edit}/></button> }
             }
        } else {
            html! { <span/> }
        }

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
        <td class="label" width="20">{"Тэг свойства"}</td>
        <td>{ tag_view }</td>
        <td class="img">{ img_view }</td>
        </table>
    }
    
}