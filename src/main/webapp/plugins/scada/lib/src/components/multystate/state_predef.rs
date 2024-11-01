use common_model::{traits::PredefStyle, utils::{filter_state_mxstyle, map_to_svg_style, merge_mx_styles, mx_style_to_map}};
use yew::{function_component, html, use_memo, use_state, AttrValue, Callback, Html, MouseEvent, Properties};
use yewdux::use_store;

use crate::{
    components::multystate::state_rect:: StateSampleRect, 
    store::{self, cell::ApplyPredefStateStyleAction},
};

#[derive(Properties, PartialEq, Debug)]
pub struct StatePredefProps<T> 
where
    T: PartialEq + PredefStyle + Clone +'static,
{
    pub value: T,
}

#[function_component]
pub fn StatePredefComponent<T>(StatePredefProps {value, }: &StatePredefProps<T>) -> Html 
where
    T: PartialEq + PredefStyle + Clone +'static,
{
    let (cell_state, cell_state_dispatch) = use_store::<store::cell::State>();  // cell meta storage

    let my_state = use_state(|| value.clone());
    // {
    //     let my_state = my_state.clone();
    //     use_effect_with(value.clone(), move |value| {
    //         my_state.dispatch(StateAction::Clone((*value).clone()));
    //     });
    // }

    let radio_id = use_memo(value.clone(),|v|AttrValue::from(v.get_radio_id().to_string()));

    let css_string = use_memo(my_state.get_style(), |style| {
            let map = mx_style_to_map(style);       
            AttrValue::from(map_to_svg_style(map).to_string())
        });

    // ================ events ========================
    let on_radio_click = {
            // let select = select.clone();
            let cell_state = cell_state.clone();
            let my_state = my_state.clone();
            Callback::from(move |_: MouseEvent| { 
                if let Ok(style) = cell_state.get_cell_style() {
                    let my_style = my_state.get_style();
                    let style = merge_mx_styles(&my_style, &style);
                    cell_state.set_cell_style(style.to_string());
                } 
            })
        };


    // ============= view items ===================
    let view_mode = html! {
            <table>
            <tr>
                <td width="100%">{ my_state.get_name().as_str() }</td>
                <td width="50"><StateSampleRect style={(*css_string).clone()}/></td>
            </tr>
            </table>    
        };

    // item view
    html! {
        <table class="prop-table">
            <td>{ view_mode }</td>
            <td class="img"><input type="radio" id={(*radio_id).clone()} name="style-selector" value={(*radio_id).clone()} onclick={on_radio_click}/></td>
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
}

#[function_component]
pub fn StatePredefEditComponent<T>(StatePredefEditProps {value }: &StatePredefEditProps<T>) -> Html 
    where
        T: PartialEq + PredefStyle + Clone +'static,
{
    let (cell_state, cell_state_dispatch) = use_store::<store::cell::State>();  // cell meta storage
    let selected = use_state(|| false);

    let my_state = use_state(|| value.clone());
    // {
    //     let my_state = my_state.clone();
    //     use_effect_with(value.clone(), move |value| {
    //         my_state.dispatch(StateAction::Clone((*value).clone()));
    //     });
    // }

    let css_string = use_memo(my_state.get_style(), |style| {
        let map = mx_style_to_map(style);       
        AttrValue::from(map_to_svg_style(map).to_string())
    });

    // =========== events ================
    let toggle_edit = {
        // let my_state = my_state.clone();
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
        let cell_state_dispatch = cell_state_dispatch.clone();
        let cell_state = cell_state.clone();
        let my_state = my_state.clone();
        let selected = selected.clone();
        Callback::from(move |_: MouseEvent| { 
            if let Some(style) = cell_state.get_cell_style().ok() {
                let filtered_style = filter_state_mxstyle(style.as_str());
                cell_state_dispatch.apply(ApplyPredefStateStyleAction{ r#type: my_state.get_type(), style: filtered_style });
            } 
            selected.set(false);           
        })
    };   

    // ============= view items =======================
    let img = {
        if *selected { 
            html! { <img src="images/close.png" onclick={toggle_close}/> }
        } else {
            html! { <img src="images/edit16.png" onclick={toggle_edit}/> }
        }
    };

    let view_mode = html! {
        <table>
        <tr>
            <td width="100%">{ my_state.get_name().as_str() }</td>
            <td width="50"><StateSampleRect style={(*css_string).clone()}/></td>
        </tr>
        </table>    
    };


    let edit_mode = html! {
        <table>
        <tr>
            <td width="100%">{ my_state.get_name().as_str() }</td>
            <td width="50"><StateSampleRect style={(*css_string).clone()}/></td>
            <td width="20">
                <button onclick={toggle_check}><img src="images/checkmark.gif" class="img-16"/></button>
            </td>
        </tr>
        </table>    
    };

    // item view
    html! {
        <table class="prop-table">
            <td>
            if *selected {
                { edit_mode }
            } else {
                { view_mode }
            }
            </td>
            <td class="img">{img}</td>
        </table>
    }
    
}
