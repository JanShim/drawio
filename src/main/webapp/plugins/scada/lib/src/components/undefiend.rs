use yew::prelude::*;
use common_model::undefiend::UndefiendXml;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};

use crate::{components::shared::{MdIcon, MdIconType}, model::cell_meta::CellType};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub value: UndefiendXml,
    #[prop_or_default]
    pub apply: Callback<CellType>,
}

#[function_component(UndefiendComponent)]
pub fn component(Props {value, apply}: &Props ) -> Html {

    let on_apply = {
        let apply = apply.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let form = event.target()
                .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

            if let Some(form) = form {
                if let Some(cell_type) = FormData::new_with_form(&form).ok().map(|data | Into::<CellType>::into(data)) {
                    apply.emit(cell_type);
                }
            }        
        })};

    // item view
   html!{
    <form onsubmit={on_apply}>
        <div class="flex-box-2 delim-label" >
            <select name="cell-type">
                <option value="multystate" selected={true}>{"Множ. состояний"}</option>
                <option value="value">{"Значение"}</option>
            </select>
            <button  type="submit"><MdIcon icon={MdIconType::Check}/></button>
        </div>
    </form>     
   }
    
}