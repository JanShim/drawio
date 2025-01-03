use yew::prelude::*;
use yew_hooks::use_unmount;
use implicit_clone::unsync::IString;

use common_model::{
	utils::convert_glyph_to_model_with_image,
	widget::{GlyphSized, WidgetContainerXml}
};

use crate::{
	components::{
		prop_table_tr::PropTableTr,
		shared::{decode_glyph_to_svg, InputType},
		widget::{
			glyph::{GlyphProps, WidgetGlyph},
			svg_view::SvgViewComponent
		}
	},
	errors::JSON_FORMAT_ERROR, model::{
		 cell_meta::CELL_TYPE_WIDGET_CONTAINER, widget::WidgetGlyphItem, widget_group::WidgetGroupListItemDto
	},
	store::cell::{CellInfoContext, NO_CONTEXT_FOUND},
	utils::{fetch, fetch_string, NULL_GLYPH_SIZED, NULL_GLYPH_SVG, NULL_UUID}
};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub value: WidgetContainerXml,
}

#[function_component]
pub fn WidgetContainerEdit(Props {
	value,
}: &Props) -> Html
{
	use_unmount(|| {
		log::debug!("WidgetContainer unmount");
	});

	let context = use_context::<CellInfoContext>().expect(NO_CONTEXT_FOUND);

	let group_name = use_state(|| AttrValue::from(""));

	let glyph = use_state(|| IString::from(""));
	let widget_uuid = use_state( || value.uuid.clone());

	let model_handler = use_state(|| AttrValue::from(""));
	{
		let url = context.api_url.clone();
		let widget_uuid = widget_uuid.clone();
		let group_pk = value.group.clone();
		let group_name = group_name.clone();
		let glyph = glyph.clone();
		let model_handler = model_handler.clone();
		use_effect_with((*widget_uuid).clone(), move |uuid| {
			let uuid = uuid.clone();
			wasm_bindgen_futures::spawn_local(
				async move {
					// fetch group
					match fetch::<WidgetGroupListItemDto>(format!("{url}/widget-group/{group_pk}")).await {
						Ok(group) => group_name.set(group.name),
						Err(err) => log::error!("{err}"),
					}

					// if uuid.eq(NULL_UUID) {
					// 	// let model = fetch_string(format!("{url}/widget/{NULL_UUID}/model")).await.unwrap_or(NULL_MODEL.to_owned());
					// 	let glyph_sized_str = fetch_string(format!("{url}/widget/{uuid}/glyph")).await.unwrap_or(NULL_GLYPH_SIZED.to_owned());
					// 	let glyph_sized = serde_json::from_str::<GlyphSized>(&glyph_sized_str).expect(JSON_FORMAT_ERROR);

					// 	// model_handler.set(model.into());
					// 	let model = convert_glyph_to_model_with_image(&glyph_sized);
					// 	model_handler.set(model.into());
					// 	glyph.set(AttrValue::from(glyph_sized.glyph));
					// 	return;
					// }

					// // fetch model
					// match fetch_string(format!("{url}/widget/{uuid}/model")).await {
					// 	Ok(model) => model_handler.set(model.into()),
					// 	Err(err) => log::error!("{err}"),
					// }

					// fetch glyph
					match fetch_string(format!("{url}/widget/{uuid}/glyph")).await {
						Ok(glyph_sized_str) => {
							let glyph_sized = serde_json::from_str::<GlyphSized>(&glyph_sized_str).expect(JSON_FORMAT_ERROR);
							let model = convert_glyph_to_model_with_image(&glyph_sized);
							model_handler.set(model.into());
							glyph.set(AttrValue::from(glyph_sized.glyph));
						},
						Err(err) => log::error!("{err}"),
					}


				 }
			);
		}
	)};

	let widget_list = use_state(|| Vec::<WidgetGlyphItem>::new());
	{
		let url = context.api_url.clone();
		let widget_list = widget_list.clone();
		use_effect_with(value.group.clone(), |group| {
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
        let glyph_svg = glyph.clone();
		let widget_uuid = widget_uuid.clone();
        Callback::from(move |pk_glyph: (IString, IString)| {
			let (pk, glyph) = pk_glyph;
			widget_uuid.set(pk);
			glyph_svg.set(glyph);
		})
    };

	// ======= views =============
    let widgets_view = {
		html! {
			for widget_list.iter().map(|item| {
				let props = yew::props! {GlyphProps {
					pk: item.uuid.to_string(),
					name: item.name.clone(),
					name_ru: item.name_ru.clone(),
					on_select: on_item_select.clone(),
					glyph: item.glyph.clone(),
				}};
				html!{ <WidgetGlyph ..props /> }
			})
		}
    };

    let props_table = html! {
		<table class="prop-table">
			<PropTableTr<AttrValue>
				edit_mode={ true }
				checked={ true }
				name={ format!("{CELL_TYPE_WIDGET_CONTAINER}:tag") }
				label={ "тег:" }
				value={ value.ds.tag.clone() }
				value_type={ InputType::STRING }
			/>
			<PropTableTr<AttrValue>
				edit_mode={ false }
				checked={ true }
				name={ format!("{CELL_TYPE_WIDGET_CONTAINER}:group") }
				label={ "Группа:" }
				value={ (*group_name).clone() }
				value_type={ InputType::STRING }
			/>
		</table>
	};

	let glyph_svg_view = {
			let svg = decode_glyph_to_svg(&glyph);
			html! {
				<SvgViewComponent {svg}/>
			}
		};

    html! {
        <div class="datails-panel">
			<input type="hidden"
				id={ format!("{CELL_TYPE_WIDGET_CONTAINER}:formGroup") }
				name={ format!("{CELL_TYPE_WIDGET_CONTAINER}:formGroup") }
			/>
			<input type="hidden"
				id={ format!("{CELL_TYPE_WIDGET_CONTAINER}:uuid") }
				name={ format!("{CELL_TYPE_WIDGET_CONTAINER}:uuid") }
				value={ (*widget_uuid).clone() }
			/>
			<input type="hidden"
				id={ format!("{CELL_TYPE_WIDGET_CONTAINER}:group") }
				name={ format!("{CELL_TYPE_WIDGET_CONTAINER}:group") }
				value={ value.group.clone() }
			/>
			<input type="hidden"
				id={ format!("{CELL_TYPE_WIDGET_CONTAINER}:model") }
				name={ format!("{CELL_TYPE_WIDGET_CONTAINER}:model") }
				value={ (*model_handler).clone() }
			/>
			{ props_table }

			<div class="delim-label">{ "Тип объекта" }</div>

			{ glyph_svg_view }

			<hr/>

			<div class="geSidebar" style="touch-action: none; display: block; transform-origin: left top;">
				{ widgets_view }
			</div>
        </div>
    }
}
