use yew::{function_component, html, use_effect_with, use_memo, use_state, Callback, Html, Properties };
use yew_hooks::use_list;
use yewdux::{use_selector, use_store};
use common_model::{data_source::DataSourceXml, geom_value::{GeomValueAttrXml, GeomValueXml}};

use crate::{ 
    components::{data_source::{self, DataSource}, shared::{use_list_selected, use_state_with, MdIcon, MdIconType}}, 
    model::cell_meta::CellMetaVariant, 
    store::cell::{self, NOT_CELL}
};


#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub edit_mode: bool,
    pub value: GeomValueXml,
    pub on_detals_apply: Callback<CellMetaVariant>,
}

#[function_component]
pub fn GeomValue(Props {
    edit_mode, 
    value, 
    on_detals_apply,
}: &Props ) -> Html 
{

    let (cell_state, _) = use_store::<cell::State>();  // cell meta storage
    let cell = use_selector(|st: &cell::State| st.cell.clone().expect(NOT_CELL));
    let geom_value = use_state_with(value.clone());
    let data_source = use_state(|| geom_value.attrs[0].ds.clone());

    // let geom_attrs = use_list(geom_value.attrs.clone());

    // let (selected, select_callback) = use_list_selected::<GeomValueAttrXml>();    

    // ============ events ====================
    // start apply process if true
    let start_apply = use_selector(|state: &cell::State | state.start_apply);
    {    
        let on_detals_apply = on_detals_apply.clone();
        let data_source = data_source.clone();
        let geom_value = geom_value.clone();
        // let geom_attrs = geom_attrs.clone();
        use_effect_with(*start_apply, move |start| {
            if *start {
                let height = geom_value.attrs[0].clone();
                let new_state = GeomValueXml {
                    attrs: vec![GeomValueAttrXml {
                        ds: (*data_source).clone(),
                        ..height
                    }],
                };

                let new_variant = CellMetaVariant::Geometry(new_state);
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

    // let cell_geom = use_memo(deps, f)
    // let on_attr_add = {
    //         let geom_attrs = geom_attrs.clone();
    //         Callback::from( move |_| {
    //             let new_attr = GeomValueAttrXml::default();
    //             geom_attrs.push(new_attr);
    //         })
    //     };

    // let apply_callback = {
    //     let geom_attrs = geom_attrs.clone();
    //     Callback::from(move |value: GeomValueAttrXml| {
    //         let index = geom_attrs.current().iter()
    //             .position(|o| o.get_name() == value.get_name());

    //         if let Some(index) = index {
    //             geom_attrs.update(index, value)
    //         }
    //     })
    // };        

    // =============== views ==================
    // let geom_attrs_view = {
    //         // let geom_attrs = geom_attrs.clone();
    //         let edit_mode = edit_mode.clone();
    //         geom_attrs.current().iter()
    //             .map(move |item| 
    //                 if edit_mode {
    //                     let props = yew::props!(GeomValueAttrEditProps {
    //                             edit_mode,
    //                             value: (*item).clone(),
    //                             selected: if let Some(selected) = (*selected).clone() {
    //                                 selected.get_name() == item.get_name()
    //                             } else {
    //                                 false
    //                             },
    //                             apply: apply_callback.clone(),
    //                             select: select_callback.clone(),
    //                         });
    //                     html! { <GeomValueAttrEdit ..props/> }
    //                 } else {
    //                     html!{ <GeomValueAttrView {edit_mode} value={(*item).clone()} /> }
    //                 }                 
    //             )
    //             .collect::<Vec<_>>()
    //     };

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
        // <>
        // <div class="flex-box delim-label">
        // {"Атрибуты:"}
        // if *edit_mode {
        //      <button onclick={on_attr_add}><MdIcon icon={MdIconType::Add}/></button> 
        // } 
        //  </div>

        // { geom_attrs_view }
        // </>
        <fieldset>
            <legend>{"Настройки высоты:"}</legend>
           { data_source_view }
        </fieldset>        
    }
}

// ---------------------------------------------
#[derive(Properties, PartialEq, Debug)]
pub struct GeomValueAttrViewProps {
    pub edit_mode: bool , 
    pub value: GeomValueAttrXml,
}

#[function_component]
pub fn GeomValueAttrView(GeomValueAttrViewProps { 
    edit_mode , 
    value, 
}: &GeomValueAttrViewProps
) -> Html {

    let data_source = use_memo(value.clone(), |v| v.ds.clone());

    let apply_ds = {
        let data_source = data_source.clone();
        Callback::from(move |ds: DataSourceXml| {
            // data_source.set(ds);
        })
    };   

    // =============== views ===============
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

    html! {<>
        {"атрибут: "} { value.name.clone() }
        { data_source_view  }
        <hr/>
    </>}
}

// ---------------------------------------------
#[derive(Properties, PartialEq, Debug)]
pub struct GeomValueAttrEditProps {
    pub edit_mode: bool,
    pub selected: bool,
    pub value: GeomValueAttrXml,
    pub apply: Callback<GeomValueAttrXml>,
    pub select: Callback<Option<GeomValueAttrXml>>,
}

#[function_component]
pub fn GeomValueAttrEdit(GeomValueAttrEditProps { 
    edit_mode,
    selected, 
    value, 
    apply, 
    select 
}: &GeomValueAttrEditProps
) -> Html {

    let data_source = use_memo(value.clone(), |v| v.ds.clone());

    let apply_ds = {
        let data_source = data_source.clone();
        Callback::from(move |ds: DataSourceXml| {
            // data_source.set(ds);
        })
    };   

    // =============== views ===============
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

    html! {<>
        {"атрибут: "} { value.name.clone() }
        { data_source_view  }
        <br/>
    </>}
}