use common_model::{traits::PredefStyle, utils::{filter_state_mxstyle, merge_mx_styles,}};
use yew::{function_component, html, use_effect_with, use_memo, use_state, AttrValue, Callback, Html, MouseEvent, Properties};
use yewdux::use_store;

use crate::{
    components::{multystate::state_rect:: StateSampleRect, shared::{use_css_styles, MdIcon, MdIconType}},
    store,
};

#[derive(Properties, PartialEq, Debug)]
pub struct StatePredefProps<T>
where
    T: PartialEq + PredefStyle + Clone +'static,
{
    pub value: T,
}

#[function_component]
pub fn StatePredefComponent<T>(StatePredefProps {value}: &StatePredefProps<T>) -> Html
where
    T: PartialEq + PredefStyle + Clone +'static,
{
    let (cell_state, _) = use_store::<store::cell::State>();  // cell meta storage

    let my_state = use_state(|| value.clone());
    {
        let my_state = my_state.clone();
        use_effect_with(value.clone(), move |value| {
            // my_state.dispatch(StateAction::Clone((*value).clone()));
            my_state.set((*value).clone());
        });
    }

    let radio_id = use_memo(value.clone(),|v|AttrValue::from(v.get_radio_id().to_string()));

    let css_strings = use_css_styles(my_state.get_style());

    // ================ events ========================
    let on_radio_click = {
            let cell_state = cell_state.clone();
            let my_state = my_state.clone();
            Callback::from(move |_: MouseEvent| {
                todo!()
                // if let Ok(style) = cell_state.get_cell_style() {
                //     let my_style = my_state.get_style();
                //     let style = merge_mx_styles(&my_style, &style);
                //     cell_state.set_cell_style(style.to_string());
                // }
            })
        };


    // ============= view items ===================
    let view_mode = html! {
            <table>
            <tr>
                <td width="100%">{ my_state.get_name().as_str() }</td>
                <td width="50"><StateSampleRect css_strings={(*css_strings).clone()} /></td>
            </tr>
            </table>
        };

    // item view
    html! {
        <table class="prop-table">
        <tr>
            <td>{ view_mode }</td>
            <td class="img"><input type="radio" id={(*radio_id).clone()} name="style-selector" value={(*radio_id).clone()} onclick={on_radio_click}/></td>
        </tr>
        </table>
    }

}

// ==========================================
#[derive(Properties, PartialEq, Debug)]
pub struct StatePredefEditProps<T>
where
    T: PartialEq + PredefStyle + Clone +'static,
{
    pub value: T,
    pub index: usize,
    pub apply: Callback<(usize, T)>,
}

#[function_component]
pub fn StatePredefEditComponent<T>(StatePredefEditProps {value, index, apply}: &StatePredefEditProps<T>) -> Html
    where
        T: PartialEq + PredefStyle + Clone +'static,
{
    let (cell_state, _) = use_store::<store::cell::State>();  // cell meta storage
    let selected = use_state(|| false);

    let my_state = use_state(|| value.clone());
    {
        let my_state = my_state.clone();
        use_effect_with(value.clone(), move |value| {
            my_state.set((*value).clone());
        });
    }

    let css_strings = use_css_styles(my_state.get_style());

    // =========== events ================
    let toggle_edit = {
        let selected = selected.clone();
        Callback::from(move |_: MouseEvent| {
            selected.set(true);
        })
    };

    let toggle_close = {
        let selected = selected.clone();
        Callback::from(move |_: MouseEvent| {
            selected.set(false);
        })
    };

    let toggle_check = {
        let cell_state = cell_state.clone();
        let my_state = my_state.clone();
        let selected = selected.clone();
        let apply = apply.clone();
        let index = index.clone();
        Callback::from(move |_: MouseEvent| {
            todo!()
            // if let Some(style) = cell_state.get_cell_style().ok() {
            //     let mut new_state = (*my_state).clone();
            //     new_state.set_style(filter_state_mxstyle(style.as_str()));
            //     my_state.set(new_state.clone());
            //     apply.emit((index, new_state));
            // }
            // selected.set(false);
        })
    };

    // ============= view items =======================
    let img = {
        if *selected {
            html! { <img src="images/close.png" onclick={toggle_close}/> }
        } else {
            html! { <button onclick={toggle_edit}><MdIcon icon={MdIconType::Edit}/></button> }
        }
    };

    let view_mode = html! {
        <table>
        <tr>
            <td width="100%">{ my_state.get_name().as_str() }</td>
            <td width="50"><StateSampleRect  css_strings={(*css_strings).clone()}/></td>
        </tr>
        </table>
    };


    let edit_mode = html! {
        <table>
        <tr>
            <td width="100%">{ my_state.get_name().as_str() }</td>
            <td width="50"><StateSampleRect css_strings={(*css_strings).clone()}/></td>
            <td width="20">
                <button onclick={toggle_check}><MdIcon icon={MdIconType::Check}/></button>
            </td>
        </tr>
        </table>
    };

    // item view
    html! {
        <table class="prop-table">
        <tr>
            <td>
            if *selected {
                { edit_mode }
            } else {
                { view_mode }
            }
            </td>
            <td class="img">{img}</td>
        </tr>
        </table>
    }

}
