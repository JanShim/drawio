use yew::prelude::*;
use glyph::{GlyphProps, WidgetGlyph};
use yew_hooks::{use_async_with_options, use_unmount, UseAsyncOptions};
use yewdux::{use_selector, use_store};
use implicit_clone::unsync::IString;
use svg_view::SvgViewComponent;
use common_model::{data_source::DataSourceXml, widget::WidgetContainerXml};

use crate::{
	components::{data_source::{self, DataSource}, shared::{use_my_datasource, use_state_with, MdIcon, MdIconType}}, errors::CellStateError, 
	model::{
		cell_meta::CellMetaVariant, 
		widget::WidgetGlyphItem
	}, 
	store::{
		cell::{self, SetCellModelAction, NO_CONTEXT_FOUND},
		mx_context::TMxGraphContext
	}, 
	utils::{fetch, fetch_string, NULL_UUID}
};

pub mod info_item;
pub mod svg_view;
pub mod list_item;
pub mod glyph;
pub mod glyph_svg;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub edit_mode: bool,
    pub value: WidgetContainerXml,
	pub on_detals_apply: Callback<CellMetaVariant>,
}

#[function_component]
pub fn WidgetContainer(Props { 
	edit_mode, 
	value,
	on_detals_apply 
}: &Props) -> Html 
{
	use_unmount(|| {
		log::debug!("WidgetContainer unmount");
	});  

    let my_value = use_state_with(value.clone());
    let data_source = use_my_datasource(value.clone());

    let (_, cell_store_dispatch) = use_store::<cell::State>();
	let mx_graph_context = use_context::<TMxGraphContext>().expect(NO_CONTEXT_FOUND);

    let widget = use_selector(|cell_state: &cell::State| {
		if let Ok(widget) = cell_state.meta.get_widget_container_meta() {
			return widget;
		};
		log::error!("{}", CellStateError::NotWidget);
		WidgetContainerXml::default()
	});  

    let type_edit_mode = use_state(|| false);
    let togle_type_edit = {
        let type_edit_mode = type_edit_mode.clone();
        Callback::from(move |_: MouseEvent| { 
			type_edit_mode.set(!*type_edit_mode); 
		})
    };  

    let glyph_svg = use_state(|| IString::from("<span>???</span>"));
	let widget_uuid = use_state(|| IString::from(NULL_UUID));
	{
		let url = mx_graph_context.api_url.clone();
		let cell_store_dispatch = cell_store_dispatch.clone();
		use_effect_with((*widget_uuid).clone(), |uuid| {
			let uuid = uuid.clone();
			wasm_bindgen_futures::spawn_local(
				async move { 
					let model = fetch_string(format!("{url}/widget/{uuid}/model")).await.unwrap();
					cell_store_dispatch.apply(SetCellModelAction(model.into()));
				 }
			);
		}
	)};

    let widget_list = {
		let url = mx_graph_context.api_url.clone();
		let group = (*widget).group.clone();
		use_async_with_options(
			async move { fetch::<Vec::<WidgetGlyphItem>>(format!("{url}/widget/{group}/glyphs")).await },
			UseAsyncOptions::enable_auto(),
		)
	};

	// ======= events =============
    let on_item_select = {
        let glyph_svg = glyph_svg.clone();
		let type_edit_mode = type_edit_mode.clone();
		let widget_uuid = widget_uuid.clone();
        Callback::from(move |pk_glyph: (IString, IString)| {
			let (pk, plyph) = pk_glyph;
			if *type_edit_mode {
				widget_uuid.set(pk);
				glyph_svg.set(plyph);							
			}			
        })
    };

	let on_type_apply = {
		let type_edit_mode = type_edit_mode.clone();
		let widget_uuid = widget_uuid.clone();
		let my_value = my_value.clone();
		Callback::from(move |_: MouseEvent| {
			let mut new_value = (*my_value).clone();
			new_value.uuid = (*widget_uuid).clone();
			my_value.set(new_value);
			type_edit_mode.set(false);     // togle type_edit_mode
		})
	};

	// start apply process if true
	let start_apply = use_selector(|state: &cell::State | state.start_apply);
	{    
		let on_detals_apply = on_detals_apply.clone();
		let data_source = data_source.clone();
		let my_value = my_value.clone();
		use_effect_with(*start_apply, move |start| {
			if *start {
				let new_value = WidgetContainerXml { 
						ds: (*data_source).clone(),
						..(*my_value).clone()
					};

				log::debug!("{new_value:?}");

				let new_variant = CellMetaVariant::WidgetContainer(new_value);
				log::debug!("NEW WIDGET CONTAINER {:?}", new_variant);      
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


    // ------------ View Items
    // let data_source_view = {
    //     let props = yew::props!(data_source::Props {
    //         ds: widget.ds.clone(),
    //         edit_mode: *edit_mode,
    //     });
    //     html! {<DataSourceComponent ..props/>}
    // };    

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

    let svg_view = {
        let inner_svg = glyph_svg.clone();
        let props = yew::props!(svg_view::Props { html: (*inner_svg).clone(), });
        html! {<SvgViewComponent ..props/>}
    };

    let img_view = {
        let edit_mode = edit_mode.clone();
		let type_edit_mode = type_edit_mode.clone();
        if edit_mode {
            if *type_edit_mode { 
                html! { <button onclick={on_type_apply}> <MdIcon icon={MdIconType::Check}/></button> }
             } else {
                html! { <button onclick={togle_type_edit}><MdIcon icon={MdIconType::Edit}/></button> }
             }
        } else {
            html! { <span/> }
        }
    };    	

    let widgets_view = {
		let on_item_select = on_item_select.clone();
        if widget_list.loading {
            html! { "Loading, wait a sec..." }
        } else  {
            widget_list.data.as_ref().map_or_else(
                || html! {},        // default
                |repo| html! { 
                    for repo.iter().map(|item: &WidgetGlyphItem| {
						let props = yew::props! {GlyphProps {
							pk: item.uuid.to_string(),
							on_select: on_item_select.clone(),
							glyph: item.glyph.clone(),
						}};
                        html!{ <WidgetGlyph ..props /> }
					})
            })      
        }   
    };	

    html! {
        <>
        // <pre>{ format!("{:?}", *multy_state) }</pre>
        <hr/>
        { data_source_view }
        <hr/>
        // <svg style="left: 1px; top: 1px; width: 124px; height: 120px; display: block; position: relative; overflow: hidden; pointer-events: none;"
        // viewBox="0 0 32 30">
        // <g style="pointer-events: none;">
        //     <g style="pointer-events: none;">
        //     </g>
        //     <g style="pointer-events: none;">
        //         <g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
        //             <path d="M 1.5 6.3 L 16 15 L 1.5 23.7 Z M 30.5 6.3 L 16 15 L 30.5 23.7 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
        //             </path>
        //         </g>
        //     </g>
        //     <g style="pointer-events: none;">
        //     </g>
        //     <g style="pointer-events: none;">
        //     </g>
        // </g>
        // </svg>
        <div class="flex-box delim-label">{"Тип объекта"}  {img_view} </div>
        { svg_view }
        <hr/>

        <div style="display: block;">
	<div class="geSidebar" style="touch-action: none; display: block; transform-origin: left top;">
		{ widgets_view }
	</div>
</div>

        
        </>
    }
}    