use common_model::geom_value::GeomValueXml;

use yew::{function_component, html, Callback, Html,  Properties, };


use crate::{ model::cell_meta::CellMetaVariant, 
store::cell};


#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub edit_mode: bool,
    #[prop_or_default]
    pub value: GeomValueXml,
    pub on_detals_apply: Callback<CellMetaVariant>,
}

#[function_component]
pub fn GeomValueComponent(Props {
    edit_mode, 
    value, 
    on_detals_apply,
    // meta,
}: &Props ) -> Html 
{

    // =============== views ==================
    html! {
        { "AAAAAAAAAA" }
    }
}