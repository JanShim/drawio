use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;

use crate::model::cell_meta::{undefiend::UndefiendMeta, CellType};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub value: UndefiendMeta,
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
            <button  type="submit"><img src="images/checkmark.gif" width="16" height="16"/></button>
        </div>
    </form>     
   }
    
}