use common_model::multystate::{range::{RangeType, RangeValue}, state::StateXml};
use implicit_clone::unsync::IString;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::{function_component, html, AttrValue, Callback, Event, Html, Properties};
use yew_hooks::UseListHandle;

use crate::components::shared::{MdIcon, MdIconType};

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
                let new_states: Vec<(usize, StateXml)> = match range_type {
                    RangeType::DISCRET => {
                        let index = states.current().len();
                        let name: AttrValue = format!("state-{index}").into();      
                        let prev_val = states.current().last()
                            .map(|o| o.value.get_value())
                            .unwrap_or(0);

                        vec![
                        (
                            index,
                            StateXml { 
                                pk: index, 
                                name,
                                value: RangeValue::DiscretConst { value: prev_val },
                                ..Default::default() 
                            }
                        )]
                    },
                    RangeType::RANGE => {
                        let mut items: Vec<(usize, StateXml)> = vec![];
                        let mut pk = states.current().len();
                        // если это первая вставка, то нужно вставить from==-inf
                        if pk == 0 {
                            let name: AttrValue = format!("range-{pk}").into();      
                            let prev_val = f32::MIN;

                            items.push((0, StateXml { 
                                pk, 
                                name,
                                value: RangeValue::RangeConst { from: prev_val },
                                ..Default::default() 
                            }));
                            pk += 1;                            
                        }

                        let name: AttrValue = format!("range-{pk}").into();      
                        let prev_val = if pk == 1 {
                                0.0
                            } else {
                                states.current().first().map(|o| o.value.get_from()).unwrap_or(0.0)
                            };
        
                        items.push((0, StateXml { 
                                pk, 
                                name,
                                value: RangeValue::RangeConst { from: prev_val },
                                ..Default::default() 
                            }
                        ));

                        // return
                        items
                    },            
                };
            log::debug!("Add {range_type:?}: {new_states:?}");

            for (i, state) in new_states {
                states.insert(i, state);
            }
        })
    };

    let on_change = {
            let on_range_type_change = on_range_type_change.clone();
            Callback::from(move |e: Event| {
                e.target()
                    .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
                    .and_then(|input| {
                        let t: IString = input.value().into();
                        let range_type: RangeType = t.into();
                        on_range_type_change.emit(range_type).into()
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
                        let selected = *o == *range_type;
                        html! { <option value={o.get_name()} {selected}>{ o.get_label() }</option> }
                    })
                    .collect::<Vec<_>>()
            }
            </select>
             <button onclick={on_state_add}><MdIcon icon={MdIconType::Add}/></button> 
        } 
         </div>


    }
}