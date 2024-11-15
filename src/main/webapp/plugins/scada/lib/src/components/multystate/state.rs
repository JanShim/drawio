use common_model::{multystate::{range::RangeType, state::StateXml}, utils::{filter_state_mxstyle, map_to_svg_style, map_to_svg_text_style, mx_style_to_map}};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};
use yew::{function_component, html, use_effect_with, use_memo, use_state, AttrValue, Callback, Html, MouseEvent, Properties, SubmitEvent};
use yewdux::use_store;

use crate::{components::{multystate::state_rect::StateSampleRect, shared::{use_css_styles, MdIcon, MdIconType}}, store};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub range_type: RangeType,
    pub value: StateXml,
}

#[function_component]
pub fn MultystateStateComponent(Props { range_type, value, }: &Props) -> Html 
{
    let my_state = use_state(|| value.clone());
    {
        let my_state = my_state.clone();
        use_effect_with(value.clone(), move |value | {
            my_state.set((*value).clone());
        });
    }

    let style_string = use_memo(my_state.style.clone(), |style| {
        let map = &mx_style_to_map(style); 
        let style = map_to_svg_style(map);
        let text_style = map_to_svg_text_style(map);
        (AttrValue::from(style.to_string()), AttrValue::from(text_style.to_string()))
    });


    // --- view items
    let view_mode = html! {
        <table class="prop-table">
        <tr>
            <td><div class="state-name">{ my_state.name.clone() }</div></td>
            <td>
            {match range_type {
                RangeType::DISCRET => html! {<>
                    {"знач: "}
                    { my_state.value.to_string() }
                </>},
                RangeType::RANGE => {
                    if my_state.pk == 0 {
                        html! {"нет нижней границы"}
                    } else {
                        html! {<>{"нижняя граница: "}{ my_state.value.to_string() }</>}
                    }
                },

            }}
            </td>
            <td><StateSampleRect style={(*style_string).0.clone()} text_style={(*style_string).1.clone()}/></td>
        </tr>
        </table>    
    };

    // item view
    html! {
        <table class="prop-table">
        <td>{ view_mode }</td>
        <td class="img"></td>
        </table>
    }
    
}


// =====================================
#[derive(Properties, PartialEq, Debug)]
pub struct MultystateStateEditProps {
    pub selected: bool,
    pub value: StateXml,
    pub apply: Callback<StateXml>,
    pub select: Callback<Option<StateXml>>,
}

#[function_component]
pub fn MultystateStateEditComponent(MultystateStateEditProps {
    value, 
    apply,
    select, 
    selected,
}: &MultystateStateEditProps) -> Html 
{
    // let (_, store_state_dispatch) = use_store::<cell::State>();
   
    let (cell_state, _) = use_store::<store::cell::State>();  // cell meta storage
    let range_type = use_state(|| Into::<RangeType>::into(value.value.clone()));

    let my_state = use_state(|| value.clone());
    {
        let my_state = my_state.clone();
        use_effect_with(value.clone(), move |value| {
            my_state.set((*value).clone());
        });
    }

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

    let css_string = use_css_styles(my_state.style.clone());

    let form_onsubmit = {
            let cell_state = cell_state.clone();
            let apply = apply.clone();
            let select = select.clone();
            Callback::from(move |event: SubmitEvent| {
                event.prevent_default();

                let form = event.target()
                    .and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

                if let Some(form) = form {
                    if let Some(state_meta) = FormData::new_with_form(&form).ok().map(|data | Into::<StateXml>::into(data)) {
                        if let Some(style) = cell_state.get_cell_style().ok() {
                            let filtered_style = filter_state_mxstyle(style.as_str());
                            let meta = StateXml {
                                style: filtered_style,
                                ..state_meta
                            };
                            apply.emit(meta);
                        }
                    }
                }
                select.emit(None);  // remove selection
            })
        };       

    // --- view items
    let button = {
        if *selected { 
            html! { <button onclick={toggle_close}><MdIcon icon={MdIconType::Cancel}/></button> }
        } else {
            html! { <button onclick={toggle_edit}><MdIcon icon={MdIconType::Edit}/></button> }
        }
    };

    // item view
    html! {
        <table class="prop-table">
        <td>{ 
            if *selected {
                html! { <StateEdit 
                        range_type={(*range_type).clone()} 
                        state={(*my_state).clone()} 
                        style={(*css_string).clone()} 
                        {form_onsubmit}/>
                }
            } else {
                html! { <StateView 
                    range_type={(*range_type).clone()} 
                    state={(*my_state).clone()} 
                    styles={(*css_string).clone()}/>   
                }
            }
         }</td>

        <td class="img" valign="top">{ button }</td>
        </table>
    }
    
}



// =====================================
#[derive(Properties, PartialEq, Debug)]
pub struct StateViewProps {
    pub range_type: RangeType,
    pub state: StateXml,
    pub styles: (AttrValue, AttrValue),
}

#[function_component]
pub fn StateView(StateViewProps {range_type, state, styles }: &StateViewProps) -> Html 
{
    html!{
        <table class="prop-table">
        <tr>
            <td><div class="state-name">{ state.name.clone() }</div></td>
            <td width="100%">
            {
                match *range_type {
                    RangeType::DISCRET => html! {<>
                        {"знач: "}
                        { state.value.to_string() }
                    </>},
                    RangeType::RANGE => {
                        if state.pk == 0 {
                            html! {"нет нижней границы"}
                        } else {
                            html! {<>{"нижняя граница: "}{ state.value.to_string() }</>}
                        }
                    },
                }
            }
            </td>
            <td><StateSampleRect style={ (*styles).0.clone() } text_style={ (*styles).1.clone() }/></td>
        </tr>
        </table>
    }
}

// =====================================
#[derive(Properties, PartialEq, Debug)]
pub struct StateEditProps {
    pub range_type: RangeType,
    pub state: StateXml,
    pub style: (AttrValue, AttrValue),
    pub form_onsubmit: Callback<SubmitEvent>,
}

#[function_component]
pub fn StateEdit(StateEditProps { 
    range_type, 
    state, 
    style, 
    form_onsubmit
}: &StateEditProps) -> Html 
{
    let init_value: AttrValue = state.value.to_string().into();
    html!{
    <form onsubmit={ form_onsubmit } class="input-form">
        <input type="hidden" id="pk" name="pk" value={state.pk.to_string()}/>
        <input type="hidden" id="range-type" name="range-type" value={(*range_type).to_string()}/>
        <table class="prop-table">
            <tr>
                <td><input id="name" name="name" value={ format!("{}", state.name) } class="state-name"/></td>
                <td width="100%">
                {match range_type {
                    RangeType::DISCRET => html! {<>
                        {"знач: "}
                        <input type="number" id="value" name="value" value={init_value.clone()} min={format!("{init_value}")} step="1" class="state-val"/>
                    </>},
                    RangeType::RANGE => {
                        if state.pk == 0 {
                            html! {<>
                                {"нет нижней границы"}
                                <input type="hidden" id="from" name="from" value={init_value.clone()} />
                            </>} 
                        } else {
                            html! {<>
                                {"нижняя граница: "}
                                <input type="number" id="from" name="from" value={init_value.clone()} min={format!("{init_value}")} step="0.01" class="state-val"/>
                            </>}
                        }
                    },
                }}
                </td>
                <td><StateSampleRect style={(*style).0.clone()} text_style={(*style).0.clone()}/></td>
                <td><button type="submit"><MdIcon icon={MdIconType::Check}/></button></td>
            </tr>
        </table>
    </form>
    }
}