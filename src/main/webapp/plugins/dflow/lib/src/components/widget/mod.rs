use yew::prelude::*;
use glyph::{GlyphProps, WidgetGlyph};
use yew_hooks::use_unmount;
use yewdux::{use_selector, use_store};
use implicit_clone::unsync::IString;
use svg_view::SvgViewComponent;
use common_model::{data_source::DataSourceXml, dflow_cell::DFlowVariant, widget::WidgetContainerXml};

use crate::{
	components::{data_source::{self, DataSource}, shared::{use_my_datasource, use_state_with, MdIcon, MdIconType}},
	model::widget::WidgetGlyphItem,
	store::{
		cell::{self, SetCellModelAction, NO_CONTEXT_FOUND},
		mx_context::TMxGraphContext
	},
	utils::{fetch, fetch_string, NULL_GLYPH, NULL_MODEL, NULL_UUID}
};

pub mod info;
pub mod svg_view;
pub mod list_item;
pub mod glyph;
pub mod glyph_svg;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub edit_mode: bool,
    pub value: WidgetContainerXml,
	pub on_detals_apply: Callback<DFlowVariant>,
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

    let type_edit_mode = use_state(|| false);
    let togle_type_edit = {
        let type_edit_mode = type_edit_mode.clone();
        Callback::from(move |_: MouseEvent| {
			type_edit_mode.set(!*type_edit_mode);
		})
    };

	let glyph_svg = use_state(|| IString::from(""));
	let widget_uuid = {
		let my_value = my_value.clone();
		use_state(move || my_value.uuid.clone())
	};
	{
		let url = mx_graph_context.api_url.clone();
		let cell_store_dispatch = cell_store_dispatch.clone();
		let my_value = my_value.clone();
		let glyph_svg = glyph_svg.clone();
		use_effect_with(my_value.uuid.clone(), |uuid| {
			let uuid = uuid.clone();
			wasm_bindgen_futures::spawn_local(
				async move {
					if uuid.eq(NULL_UUID) {
						let model = fetch_string(format!("{url}/widget/{NULL_UUID}/model")).await.unwrap_or(NULL_MODEL.to_owned());
						let glyph = fetch_string(format!("{url}/widget/{uuid}/glyph")).await.unwrap_or(NULL_GLYPH.to_owned());
						cell_store_dispatch.apply(SetCellModelAction(model.into()));
						glyph_svg.set(AttrValue::from(glyph));
						return;
					}

					log::debug!("my_value : {:?}", *my_value);

					// fetch model
					match fetch_string(format!("{url}/widget/{uuid}/model")).await {
						Ok(model) => cell_store_dispatch.apply(SetCellModelAction(model.into())),
						Err(err) => log::error!("{err}"),
					}

					// fetch glyph
					match fetch_string(format!("{url}/widget/{uuid}/glyph")).await {
						Ok(glyph) => glyph_svg.set(AttrValue::from(glyph)),
						Err(err) => log::error!("{err}"),
					}
				 }
			);
		}
	)};

	let widget_list = use_state(|| Vec::<WidgetGlyphItem>::new());
	{
		let url = mx_graph_context.api_url.clone();
		let widget_list = widget_list.clone();
		use_effect_with(my_value.group.clone(), |group| {
			let group = group.clone();
			wasm_bindgen_futures::spawn_local(async move {
				match fetch::<Vec::<WidgetGlyphItem>>(format!("{url}/widget/{group}/glyphs")).await {
					Ok(list) => widget_list.set(list),
					Err(err) => log::error!("{err}"),
				};
			});
		});
	}

	// ======= events =============
    let on_item_select = {
        let glyph_svg = glyph_svg.clone();
		let type_edit_mode = type_edit_mode.clone();
		let widget_uuid = widget_uuid.clone();
        Callback::from(move |pk_glyph: (IString, IString)| {
			let (pk, glyph) = pk_glyph;
			if *type_edit_mode {
				widget_uuid.set(pk);
				glyph_svg.set(glyph);
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

				let new_variant = DFlowVariant::WidgetContainer(new_value);
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
	// 	let data_source = data_source.clone();
	// 	let apply_ds = apply_ds.clone();
	// 	let props = yew::props!(data_source::Props {
	// 		ds: (*data_source).clone(),
	// 		edit_mode: *edit_mode,
	// 		on_apply: apply_ds,
	// 	});
	// 	html! {<DataSource ..props/>}
	// };

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
		html! {
			for widget_list.iter().map(|item| {
				let props = yew::props! {GlyphProps {
					pk: item.uuid.to_string(),
					on_select: on_item_select.clone(),
					glyph: item.glyph.clone(),
				}};
				html!{ <WidgetGlyph ..props /> }
			})
		}
    };

    html! {
        <>
        // <hr/>
        // { data_source_view }
        // <hr/>

		<div class="flex-box delim-label">{"Тип объекта"}
			{img_view}
		</div>

		<SvgViewComponent glyph={(*glyph_svg).clone()}/>
        <hr/>

		<div class="geSidebar" style="touch-action: none; display: block; transform-origin: left top;">
			{ widgets_view }
		</div>
        </>
    }
}