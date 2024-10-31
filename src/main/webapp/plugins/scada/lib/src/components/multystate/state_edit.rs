use common_model::multystate::{range::RangeType, state::StateXml};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};
use yew::{html, function_component, use_memo, use_state, AttrValue, Callback, Html, MouseEvent, Properties, SubmitEvent};
use yewdux::use_store;

use crate::{
    components::multystate::state_rect::StateSampleRect, 
    store::{self, cell::MultystateApplyStateAction}, utils::{map_to_svg_style, mx_style_to_map} 
};


#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub selected: bool,
    pub value: StateXml,
    pub select: Callback<Option<StateXml>>,
}

#[function_component(MultystateStateEditComponent)]
pub fn component(Props {
    value, 
    select, 
    selected
}: &Props) -> Html {
    // cell meta storage
    let (cell_state, cell_state_dispatch) = use_store::<store::cell::State>();
    let range_type = use_state(|| Into::<RangeType>::into(value.value.clone()));

    let my_state = use_state(|| value.clone());
    // {
    //     let my_state = my_state.clone();
    //     use_effect_with(value.clone(), move |value| {
    //         // my_state.dispatch(StateAction::Clone((*value).clone()));
    //         my_state.set((*value).clone());
    //     });
    // }

    let toggle_edit = {
        let my_state = my_state.clone();
        let select = select.clone();
        Callback::from(move |_: MouseEvent| { select.emit(Some((*my_state).clone())) })
    };      

    let toggle_close = {
        let select = select.clone();
        Callback::from(move |_: MouseEvent| { 
            select.emit(None);  // remove selection
        })
    };   

    let style_string = use_memo(my_state.style.clone(), |style| {
        let map = mx_style_to_map(style);       
        AttrValue::from(map_to_svg_style(map))
    });


    let form_onsubmit = {
        let cell_state_dispatch = cell_state_dispatch.clone();
        let cell_state = cell_state.clone();
        let select = select.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let form = event.target()
                .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

            if let Some(form) = form {
                if let Some(state_meta) = FormData::new_with_form(&form).ok().map(|data | Into::<StateXml>::into(data)) {
                    if let Some(style) = cell_state.get_cell_style().ok() {
                        let meta = StateXml {
                            style,
                            ..state_meta
                        };
                        cell_state_dispatch.apply(MultystateApplyStateAction(meta));
                    }
                }
            }
            select.emit(None);  // remove selection
        })
    };       

    // --- view items
    let edit_mode_view = html! {
        <table>
        <tr>
            <td width="200">{ my_state.name.as_str() }</td>
            <td>{"знач: "}</td>
            <td width="35">{ my_state.value.to_string() }</td>
            <td width="50"><StateSampleRect style={(*style_string).clone()}/></td>
        </tr>
        </table>
    };

    let edit_mode_edit = {
        let pk = value.pk;
        let init_value = value.value.to_string();
        let range_type = range_type.clone();

        html! {
        <form onsubmit={ form_onsubmit } class="input-form">
            <input type="hidden" id="pk" name="pk" value={pk.to_string()}/>
            <input type="hidden" id="range-type" name="range-type" value={(*range_type).to_string()}/>
            if (*range_type)==RangeType::RANGE {
                <input type="hidden" id="from" name="from" value={value.value.get_from().to_string()}/>
            }

            <table>
                <tr>
                    <td width="200">
                        <input id="name" name="name" value={ format!("{}", my_state.name) } />
                    </td>
                    <td>{"знач: "}</td>
                    <td width="35">
                        <input type="number" id="value" name="value" value={init_value.clone()} min={format!("{init_value}")} step="1" />
                    </td>
                    <td width="50"><StateSampleRect style={(*style_string).clone()}/></td>
                    <td width="20">
                        <button type="submit"><img src="images/checkmark.gif" class="img-16"/></button>
                    </td>
                </tr>
            </table>
        </form>
    }};

    let img = {
        if *selected { 
            html! { <img src="images/close.png" onclick={toggle_close}/> }
        } else {
            html! { <img src="images/edit16.png" onclick={toggle_edit}/> }
        }
    };

    // item view
    html! {
        <table class="prop-table">
        // <td class="label" width="10">{my_state.pk.clone()}</td>
        <td>{ 
            if *selected {
                { edit_mode_edit }
            } else {
                { edit_mode_view }
            }
         }</td>
        <td class="img" valign="top">{ img }</td>
        </table>
    }
    
}