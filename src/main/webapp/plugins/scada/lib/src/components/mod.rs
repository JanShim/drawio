use yew::{prelude::*, virtual_dom::VNode};
use diagram::info_item::DiagramInfoComponent;
use widget::info_item::WidgetInfoComponent;
use stylist::{css, yew::Global};
use yewdux::use_selector;

use crate::model::common::ModelForm;
use crate::store;

pub mod diagram;
pub mod widget;
pub mod cell_details;
pub mod multystate;
pub mod free_value;
pub mod undefiend;


pub fn get_global_css() -> VNode {
    html! {
        <Global css={css!(r#"
        .prop-edit-panel {
            background-color: rgb(237, 244, 255);
            min-height: 20px;
        }
        
        .flex-box {
            display:flex;
            justify-content:space-between;
        }
        
        .flex-box-2 {
            display:flex;
            justify-content: flex-end;
        }
        
        .delim-label {
            background-color: #e9e9e9;
            height: 25px;
            padding: 3px;
        }    
            
        table.prop-table {
            width: 100%;
        }
        table.prop-table td {
            padding: 0px 5px 0px 5px;
            height: 25px;
            vertical-align: middle;
        }
        table.prop-table td input {
            width: 100%;
            height: 16px;
        }
        table.prop-table td.label {
            background-color: rgb(221, 221, 221);
            text-align: right;
        }
        table.prop-table td.img {
            width: 16px;
            padding: 0px;
        }     
        
        .img-16 {
            width: 16px;
            height: 16px;
            padding: 0px;
        }
        
        form.input-form input {
            margin: 0px 5px 0px 5px;
        }    
        
        div.svg-view {
            text-align: center;
        }

        .input-100 {
            width: 100%;
        }

        div.label {
            font-weight: bold;
        }

        div.value {
            padding: 2px;
        }

        input.state-val {
            width: 30px !important;
        }
        
        "#)} />
    }
}

#[function_component(InfoComponent)]
pub fn diagram_info_component() -> Html {
    let model_meta = use_selector(|state: &store::diagram::State| {
        state.model_meta.clone()
    });

    match *model_meta {
        ModelForm::Diagram(_) => html! { <DiagramInfoComponent/> },
        ModelForm::Widget(_) =>  html! { <WidgetInfoComponent/>  },
    }
}