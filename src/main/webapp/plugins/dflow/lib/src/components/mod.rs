use yew::{prelude::*, virtual_dom::VNode};
use stylist::{css, yew::Global};
use yewdux::use_selector;
use widget::info::WidgetInfoComponent;
use diagram::info::DiagramInfoComponent;


use crate::model::common::ModelForm;
use crate::store;

pub mod diagram;
pub mod widget;
pub mod cell_details;
pub mod multystate;
pub mod label_value;
pub mod geom_value;
pub mod shared;
pub mod data_source;
pub mod prop_table_tr;


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

.flex-cell {
    display: flex;
    gap: 4px;
    flex-direction: row;
    vertical-align: middle;
}


.delim-label {
    background-color: #e9e9e9;
    height: 25px;
    padding: 3px;
}

table.prop-table {
    width: 100%;
    border: 1px solid black;
    border-collapse: collapse;
}
table.prop-table td {
    padding: 0px 5px 0px 5px;
    height: 25px;
    vertical-align: middle;
    border: 1px solid black;
    border-collapse: collapse;
}
table.prop-table td input {
    height: 16px;
}
table.prop-table td.label {
    background-color: rgb(221, 221, 221);
    width: 40px;
    text-align: right;
}
table.prop-table td.img {
    width: 16px;
    padding: 0px;
}

.material-icons.md-16 {
    font-size: 16px;
}

.img-16 {
    width: 16px;
    height: 16px;
    padding: 0px;
}

div.svg-view {
    text-align: center;
    padding: 5px;
}

.input-100 {
    width: 98%;
}

div.label {
    font-weight: bold;
}

div.value {
    padding: 2px;
}

.prop-value {
    width: 230px;
    overflow-x: auto;
    white-space: nowrap;
}

input.state-val {
    width: 45px !important;
}

.types-list label {
  font: 1rem 'Fira Sans', sans-serif;
}

.types-list input {
  margin: 0.4rem;
}

.state-name {
    width: 100px;
}

.state-name > input {
    width: 100%;
}

.input-valign-center {
    height: 30px;
    display: flex;
    align-items: center
}

.datails-panel {
    border-bottom: 1px solid var(--border-color);
}

.item-details button {
    padding: 2px;
}

"#)} />
}
}

#[function_component]
pub fn InfoComponent() -> Html {
    let model_meta = use_selector(|state: &store::diagram::State| {
        state.model_meta.clone()
    });

    match (*model_meta).clone() {
        ModelForm::Diagram(form) => html! { <DiagramInfoComponent {form}/> },
        ModelForm::Widget(form) =>  {
            html! { <WidgetInfoComponent {form}/> }
        },
    }
}