use common_model::multystate::{range::{RangeType, RangeValue}, state::StateXml};
use implicit_clone::unsync::IString;
use quick_xml::de;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::{function_component, html, use_state, AttrValue, Callback, Event, Html, Properties};
use yew_hooks::UseListHandle;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub edit_mode: bool,
    pub range_type: RangeType,
    pub states: UseListHandle<StateXml>,
    pub on_range_type_change: Callback<RangeType>,
}

#[function_component]
pub fn StatesSelector(Props {
    edit_mode,
    states,
    range_type, 
    on_range_type_change 
}: &Props) -> Html  
{
    let range_types = vec![RangeType::DISCRET, RangeType::RANGE];

    // let range_type = use_state(|| range_type.clone());

    // ========= events ===================
    let on_state_add = {
        let range_type = range_type.clone();
        let states = states.clone();
        Callback::from(move |_| {
            let index = states.current().len();
            let name: AttrValue = format!("state-{index}").into();      

            let new_state = match range_type {
                RangeType::DISCRET => {
                    let prev_val = states.current().last()
                        .map(|o| o.value.get_value())
                        .unwrap_or(0);
                    StateXml { 
                        pk: index, 
                        name,
                        value: RangeValue::DiscretConst { value: prev_val },
                        ..Default::default() 
                    }
                },
                RangeType::RANGE => {
                    let prev_val = states.current().last()
                        .map(|o| o.value.get_to())
                        .unwrap_or(0.0);
    
                    StateXml { 
                        pk: index, 
                        name,
                        value: RangeValue::RangeConst { from: prev_val, to: prev_val },
                        ..Default::default() 
                    }
                },            
            };
            log::debug!("Add {range_type:?}: {new_state:?}");

            states.insert(index, new_state);
        })
    };

    let on_change = {
            let on_range_type_change = on_range_type_change.clone();
            Callback::from(move |e: Event| {
                e.target()
                    .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
                    .and_then(|input| {
                        let t: IString = input.value().into();
                        let t: RangeType = t.into();
                        // range_type.set(t.clone());
                        on_range_type_change.emit(t).into()
                    });
            })
        };

    // ============== view ==================
    html! {
        <div class="flex-box delim-label">
        {"Состояния"}
        if *edit_mode {

            <select onchange={on_change} > {
                range_types.iter()
                    .map(|o| {
                        let selected = *o == RangeType::DISCRET;
                        html! { <option value={o.get_name()} {selected}>{ o.get_label() }</option> }
                    })
                    .collect::<Vec<_>>()
            }
            </select>

            // <Select<RangeType> options={range_types} on_change={on_change} />                    

             <button onclick={on_state_add} title="Добавить">{"+"}</button> 
        } 
         </div>


    }
}