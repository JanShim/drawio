use common_model::{multystate::{range::RangeType, state::StateXml}, utils::{filter_state_mxstyle, merge_mx_styles}};
use quick_xml::de;
use yew::{function_component, html, use_context, use_memo, use_state, AttrValue, Callback, Html, MouseEvent, Properties};

use crate::{
    components::{
        multystate::{
            state_rect::StateSampleRect,
            FORM_NAME_PREFIX,
            FORM_NAME_SUFIX_FROM, FORM_NAME_SUFIX_NAME, FORM_NAME_SUFIX_PK, FORM_NAME_SUFIX_STYLE, FORM_NAME_SUFIX_VALUE
        },
        shared::{use_css_styles, use_state_with, MdIcon, MdIconType}
    },
    model::cell_meta::CELL_TYPE_MULTY,
    store::cell::{CellInfoContext, NO_CONTEXT_FOUND}, utils::refresh_cell,
};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub range_type: RangeType,
    pub value: StateXml,
}

// #[function_component]
// pub fn MultystateStateComponent(Props { range_type, value, }: &Props) -> Html
// {
//     // let my_state = use_state(|| value.clone());
//     // {
//     //     let my_state = my_state.clone();
//     //     use_effect_with(value.clone(), move |value | {
//     //         my_state.set((*value).clone());
//     //     });
//     // }
//     let my_state = use_state_with(value.clone());

//     let css_strings = use_css_styles(my_state.style.clone());

//     // --- view items
//     let view_mode = html! {
//         <table class="prop-table">
//         <tr>
//             <td><div class="state-name">{ my_state.name.clone() }</div></td>
//             <td>
//             {match range_type {
//                 RangeType::DISCRET => html! {<>
//                     {"знач: "}
//                     { my_state.value.to_string() }
//                 </>},
//                 RangeType::RANGE => {
//                     if my_state.pk == 0 {
//                         html! {"нет нижней границы"}
//                     } else {
//                         html! {<>{"нижняя граница: "}{ my_state.value.to_string() }</>}
//                     }
//                 },

//             }}
//             </td>
//             <td><StateSampleRect css_strings={(*css_strings).clone()} /></td>
//         </tr>
//         </table>
//     };

//     // item view
//     html! {
//         <table class="prop-table">
//         <tr>
//             <td>{ view_mode }</td>
//             <td class="img"></td>
//         </tr>
//         </table>
//     }

// }


// =====================================
#[derive(Properties, PartialEq, Debug)]
pub struct MultystateStateEditProps {
    pub edit_mode: bool,
    pub value: StateXml,
}

#[function_component]
pub fn MultystateStateEditComponent(MultystateStateEditProps {
    edit_mode,
    value,
}: &MultystateStateEditProps) -> Html
{
    let range_type = use_state(|| Into::<RangeType>::into(value.value.clone()));

    let my_state = use_state_with(value.clone());

    let css_strings = use_css_styles(my_state.style.clone());

    log::debug!("{}", css_strings.0);

    // ================= view items ==========================
    html! {
        <tr>
            <td colspan="2">
                if *edit_mode {
                    <StateEdit
                        range_type={(*range_type).clone()}
                        state={(*my_state).clone()}
                    />
                } else {
                    <StateView
                        range_type={(*range_type).clone()}
                        state={(*my_state).clone()}
                        css_strings={(*css_strings).clone()}
                    />
                }
            </td>
        </tr>
    }

}



// =====================================
#[derive(Properties, PartialEq, Debug)]
pub struct StateViewProps {
    pub range_type: RangeType,
    pub state: StateXml,
    pub css_strings: (AttrValue, AttrValue),
}

#[function_component]
pub fn StateView(StateViewProps {
    range_type,
    state,
    css_strings
}: &StateViewProps) -> Html
{
    let range_value = use_memo(state.clone().clone(), |v| AttrValue::from(v.value.to_string()));

    html!{
        <div class="flex-cell">
            <div class="state-name">
                { state.name.clone() }
            </div>
            <div style="margin-left: auto;">
                {match range_type {
                    RangeType::DISCRET => html! {<>
                        <span>{ "=" }</span>
                        <span style="margin-right: 50px;">{ (*range_value).clone() }</span>
                    </>},
                    RangeType::RANGE => {
                        if state.pk == 0 {
                            html! {
                                <span style="display: inline-block; width: 55px;">{ "> -∞" }</span>
                            }
                        } else {
                            html! {<>
                                <span>{ "≥" }</span>
                                <span style="margin-right: 50px;">{ (*range_value).clone() }</span>
                            </>}
                        }
                    },
                }}
            </div>
            <StateSampleRect css_strings={(*css_strings).clone()} />
        </div>
    }
}

// =====================================
#[derive(Properties, PartialEq, Debug)]
pub struct StateEditProps {
    pub range_type: RangeType,
    pub state: StateXml,
}

#[function_component]
pub fn StateEdit(StateEditProps {
    range_type,
    state,
}: &StateEditProps) -> Html
{
    let context = use_context::<CellInfoContext>().expect(NO_CONTEXT_FOUND);

    let my_state = use_state_with(state.clone());

    let range_value = use_memo(state.clone(), |v| {
            log::debug!("name: {:?}, value: {:?}", v.name, v.value.to_string());

            AttrValue::from(v.value.to_string())
        });

    let css_strings = use_css_styles(my_state.get_style());

    // =============== events =======================
    let get_style = {
        let mx_cell =  context.mx_cell.clone();
        let my_state = my_state.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            let style = mx_cell.get_style()
                .map(|o| filter_state_mxstyle(o.as_str()));

            let mut new_state = (*my_state).clone();
            new_state.set_style(style.unwrap_or_default());

            my_state.set(new_state);
        })
    };

    let set_style = {
        let mxcell =  context.mx_cell.clone();
        let mxeditor = context.mx_editor.clone();
        let my_state = my_state.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            let new_style = mxcell.get_style()
                .map(|o| merge_mx_styles(my_state.get_style().as_str(), o.as_str()));

            if let Some(new_style) = new_style {
                mxcell.set_style(new_style.to_string());

                refresh_cell(&mxeditor, &mxcell);
            }
        })
    };


    // =============== view =========================
    html!{
        <div class="flex-cell">
            <div class="state-name">
                <input type="hidden"
                    id={ get_form_name(FORM_NAME_SUFIX_PK) }
                    name={ get_form_name(FORM_NAME_SUFIX_PK) }
                    value={state.pk.to_string()}
                />
                <input
                    id={ get_form_name(FORM_NAME_SUFIX_NAME) }
                    name={ get_form_name(FORM_NAME_SUFIX_NAME) }
                    value={ format!("{}", state.name) }
                />
            </div>
            <div style="margin-left: auto;">
                {match range_type {
                    RangeType::DISCRET => html! {<>
                        <span>{ "=" }</span>
                        <input type="number"
                            id={ get_form_name(FORM_NAME_SUFIX_VALUE) }
                            name={ get_form_name(FORM_NAME_SUFIX_VALUE) }
                            min={ (*range_value).clone() }
                            step="1" class="state-val"
                            value={ (*range_value).clone() }
                        />
                    </>},
                    RangeType::RANGE => {
                        if state.pk == 0 {
                            html! {<>
                                <span style="display: inline-block; width: 55px;">{ "> -∞" }</span>
                                <input type="hidden"
                                    id={ get_form_name(FORM_NAME_SUFIX_FROM) }
                                    name={ get_form_name(FORM_NAME_SUFIX_FROM) }
                                    value={ (*range_value).clone() }
                                />
                            </>}
                        } else {
                            html! {<>
                                <span>{ "≥" }</span>
                                <input type="number"
                                    id={ get_form_name(FORM_NAME_SUFIX_FROM) }
                                    name={ get_form_name(FORM_NAME_SUFIX_FROM) }
                                    min={ (*range_value).clone() }
                                    step="0.01" class="state-val"
                                    value={ (*range_value).clone() }
                                />
                            </>}
                        }
                    },
                }}
            </div>
            <div>
                <input type="hidden"
                    id={ get_form_name(FORM_NAME_SUFIX_STYLE) }
                    name={ get_form_name(FORM_NAME_SUFIX_STYLE) }
                    value={ my_state.get_style() }
                />
                <StateSampleRect css_strings={(*css_strings).clone()} />
            </div>
            <button onclick={ set_style }><MdIcon icon={MdIconType::KeyboardDoubleArrowUp}/></button>
            <button onclick={ get_style }><MdIcon icon={MdIconType::KeyboardDoubleArrowDown}/></button>
        </div>
    }
}

// ----------------------------------
fn get_form_name(sufix: &str) -> AttrValue {
    AttrValue::from(format!("{CELL_TYPE_MULTY}:{FORM_NAME_PREFIX}-{sufix}"))
}
