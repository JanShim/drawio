use yew::prelude::*;
use base64::prelude::*;
use yew_hooks::use_unmount;
use implicit_clone::unsync::IString;
use common_model::widget::{GlyphSized, WidgetContainerXml};

use crate::{
	components::{
		prop_table_tr::PropTableTr, shared::{decode_glyph_to_svg, InputType},
		widget::svg_view::SvgViewComponent
	}, errors::JSON_FORMAT_ERROR, model::{
		cell_meta::CELL_TYPE_WIDGET_CONTAINER, widget::WidgetListItem, widget_group::WidgetGroupListItemDto
	}, store::cell::{CellInfoContext, NO_CONTEXT_FOUND}, utils::{fetch, fetch_string, NULL_GLYPH_SIZED, NULL_GLYPH_SVG, NULL_MODEL, NULL_UUID}
};


#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub value: WidgetContainerXml,
}

#[function_component]
pub fn WidgetContainerView(Props {
	value,
}: &Props) -> Html
{
    use_unmount(|| {
		log::debug!("WidgetContainer unmount");
	});

	let context = use_context::<CellInfoContext>().expect(NO_CONTEXT_FOUND);

	let meta = use_state(|| WidgetListItem::default());

	let group_name = use_state(|| value.group.clone());

	let glyph_svg = use_state(|| IString::from(""));
	{
		let url = context.api_url.clone();
		let group_pk = value.group.clone();
		let group_name = group_name.clone();
		let glyph_svg = glyph_svg.clone();
		let meta = meta.clone();
		use_effect_with(value.uuid.clone(), |uuid| {
			let uuid = uuid.clone();
			wasm_bindgen_futures::spawn_local(
				async move {
					// fetch widget meta
					match fetch::<WidgetListItem>(format!("{url}/widget/{uuid}")).await {
						Ok(widget_meta) => meta.set(widget_meta),
						Err(err) => log::error!("{err}"),
					}

					// fetch group
					match fetch::<WidgetGroupListItemDto>(format!("{url}/widget-group/{group_pk}")).await {
						Ok(group) => group_name.set(group.name),
						Err(err) => log::error!("{err}"),
					}

					// fetch glyph
					match fetch_string(format!("{url}/widget/{uuid}/glyph")).await {
						Ok(glyph_sized_str) => {
							let glyph_sized = serde_json::from_str::<GlyphSized>(&glyph_sized_str).expect(JSON_FORMAT_ERROR);
							let glyph = decode_glyph_to_svg(&glyph_sized.glyph);
							glyph_svg.set(glyph);
						},
						Err(err) => log::error!("{err}"),
					}
				 }
			);
		}
	)};


    // ------------ View Items
    let props_table = html! {
		<div>
			<table class="prop-table">
				<PropTableTr<AttrValue>
					edit_mode={ false }
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
		</div>
	};

    html! {
        <>
        <hr/>
        { props_table }
        <hr/>

		<div class="flex-box delim-label">{ "Тип объекта" }</div>

		<SvgViewComponent svg={ (*glyph_svg).clone() }/>

		<div> { meta.name_ru.clone().unwrap_or(meta.name.clone())} </div>

        </>
    }
}