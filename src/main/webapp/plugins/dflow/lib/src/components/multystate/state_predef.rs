use common_model::{traits::PredefStyle, utils::{filter_state_mxstyle, merge_mx_styles,}};
use yew::{function_component, html, use_context, use_effect_with, use_memo, use_state, AttrValue, Callback, Html, MouseEvent, Properties};
use yewdux::use_store;

use crate::{
    components::{multystate::state_rect:: StateSampleRect, shared::{use_css_styles, use_state_with, MdIcon, MdIconType}},
    store::{self, cell::{CellInfoContext, NO_CONTEXT_FOUND}}, utils::refresh_cell,
};

// #[derive(Properties, PartialEq, Debug)]
// pub struct StatePredefProps<T>
// where
//     T: PartialEq + PredefStyle + Clone +'static,
// {
//     pub value: T,
// }

// #[function_component]
// pub fn StatePredefComponent<T>(StatePredefProps {value}: &StatePredefProps<T>) -> Html
// where
//     T: PartialEq + PredefStyle + Clone +'static,
// {
//     let (cell_state, _) = use_store::<store::cell::State>();  // cell meta storage

//     let my_state = use_state(|| value.clone());
//     {
//         let my_state = my_state.clone();
//         use_effect_with(value.clone(), move |value| {
//             // my_state.dispatch(StateAction::Clone((*value).clone()));
//             my_state.set((*value).clone());
//         });
//     }

//     let radio_id = use_memo(value.clone(),|v|AttrValue::from(v.get_radio_id().to_string()));

//     let css_strings = use_css_styles(my_state.get_style());

//     // ================ events ========================
//     let on_radio_click = {
//             let cell_state = cell_state.clone();
//             let my_state = my_state.clone();
//             Callback::from(move |_: MouseEvent| {
//                 todo!()
//                 // if let Ok(style) = cell_state.get_cell_style() {
//                 //     let my_style = my_state.get_style();
//                 //     let style = merge_mx_styles(&my_style, &style);
//                 //     cell_state.set_cell_style(style.to_string());
//                 // }
//             })
//         };


//     // ============= view items ===================
//     let view_mode = html! {
//             <table>
//             <tr>
//                 <td width="100%">{ my_state.get_name().as_str() }</td>
//                 <td width="50"><StateSampleRect css_strings={(*css_strings).clone()} /></td>
//             </tr>
//             </table>
//         };

//     // item view
//     html! {
//         <table class="prop-table">
//         <tr>
//             <td>{ view_mode }</td>
//             <td class="img"><input type="radio" id={(*radio_id).clone()} name="style-selector" value={(*radio_id).clone()} onclick={on_radio_click}/></td>
//         </tr>
//         </table>
//     }

// }

// ==========================================
#[derive(Properties, PartialEq, Debug)]
pub struct StatePredefEditProps<T>
where
    T: PartialEq + PredefStyle + Clone +'static,
{
    pub edit_mode: bool,
    pub checked: bool,
    pub name: AttrValue,
    pub value: T,
}

#[function_component]
pub fn StatePredefEditComponent<T>(
    StatePredefEditProps {edit_mode, checked, name, value }: &StatePredefEditProps<T>
) -> Html
    where
        T: PartialEq + PredefStyle + Clone +'static,
{
    let context = use_context::<CellInfoContext>().expect(NO_CONTEXT_FOUND);

    let my_state = use_state_with(value.clone());

    let css_strings = use_css_styles(my_state.get_style());

    // =========== events ================
    let get_style = {
        let mxcell =  context.mx_cell.clone();
        let my_state = my_state.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            let style = mxcell.get_style()
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

    // ============= view items =======================
    // item view
    html! {
        if *edit_mode {
            <tr>
                <td colspan="2">
                    <div class="flex-cell">
                        <div>{ my_state.get_name().as_str() }</div>
                        <div style="margin-left: auto;">
                            <input type="hidden" id={ name } name={ name } value={ my_state.get_style() }/>
                            <StateSampleRect css_strings={ (*css_strings).clone() }/>
                        </div>
                        if *edit_mode {
                            <button onclick={ set_style }><MdIcon icon={MdIconType::KeyboardDoubleArrowUp}/></button>
                            <button onclick={ get_style }><MdIcon icon={MdIconType::KeyboardDoubleArrowDown}/></button>
                        }
                    </div>
                </td>
            </tr>
        } else {
            <tr>
                <td colspan="2">
                    <div class="flex-cell">
                        <div>{ my_state.get_name().as_str() }</div>
                        <div style="margin-left: auto;">
                            <StateSampleRect css_strings={ (*css_strings).clone() }/>
                        </div>
                    </div>
                </td>
            </tr>
        }

    }

}
