use yew::{function_component, html, use_effect_with, Callback, Html, Properties };
use yew_hooks::use_unmount;
use yewdux::use_selector;
use common_model::{data_source::DataSourceXml, dflow_cell::DFlowVariant, geom_value::GeomValueXml};

use crate::{ 
    components::{
        data_source::{self, DataSource}, prop_table_tr::PropTableTr, shared::{use_my_datasource, use_state_with}
    }, 
    store::cell
};


#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub edit_mode: bool,
    pub value: GeomValueXml,
    pub on_detals_apply: Callback<DFlowVariant>,
}

#[function_component]
pub fn GeomValue(Props {
    edit_mode, 
    value, 
    on_detals_apply,
}: &Props ) -> Html 
{
    use_unmount(|| {
        log::debug!("GeomValue unmount");
    });    
    
    let geom_value = use_state_with(value.clone());
    let data_source = use_my_datasource(value.clone());

    // let geom_attrs = use_list(geom_value.attrs.clone());

    // let (selected, select_callback) = use_list_selected::<GeomValueAttrXml>();    

    // ============ events ====================
    // start apply process if true
    let start_apply = use_selector(|state: &cell::State | state.start_apply);
    {    
        let on_detals_apply = on_detals_apply.clone();
        let data_source = data_source.clone();
        let geom_value = geom_value.clone();
        use_effect_with(*start_apply, move |start| {
            if *start {
                let new_value = GeomValueXml { 
                        ds: (*data_source).clone(),
                        ..(*geom_value).clone()
                    };

                log::debug!("{new_value:?}");

                let new_variant = DFlowVariant::Geometry(new_value);
                log::debug!("NEW GEOM {:?}", new_variant);      
                on_detals_apply.emit(new_variant);
            }
        })
    };    

    let apply_ds = {
        let data_source = data_source.clone();
        Callback::from(move |ds: DataSourceXml| {
            data_source.set(ds);
        })
    };     

    let on_min_commit = {
            let geom_value = geom_value.clone();
            Callback::from(move |value| {
                let mut curr = (*geom_value).clone();
                curr.min = value;
                geom_value.set(curr);
            })
        };

    let on_max_commit = {
            let geom_value = geom_value.clone();
            Callback::from(move |value| {
                let mut curr = (*geom_value).clone();
                curr.max = value;
                geom_value.set(curr);
            })
        };

    // =============== views ==================
    let data_source_view = {
            let data_source = data_source.clone();
            let apply_ds = apply_ds.clone();
            let props = yew::props!(data_source::Props {
                ds: (*data_source).clone(),
                edit_mode: *edit_mode,
                on_apply: apply_ds,
            });
            html! {<DataSource ..props/>}
        };
    

    html! {
        <fieldset>
            <legend>{"Настройки ширины:"}</legend>
           { data_source_view }

           <table class="prop-table">
           <PropTableTr<f32> {edit_mode} label={"min:"} value={(*value).min} on_commit={on_min_commit}/>
           <PropTableTr<f32> {edit_mode} label={"max:"} value={(*value).max} on_commit={on_max_commit}/>
           </table>

        </fieldset>        
    }
}
