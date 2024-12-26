use yew::prelude::*;
use yew_hooks::{use_async_with_options, use_toggle, use_unmount, UseAsyncOptions};
use implicit_clone::unsync::IString;
use common_model::widget::WidgetContainerXml;

use crate::{
	components::{prop_table_tr::PropTableTr, shared::{InputType, MdIcon, MdIconType}, widget::{glyph::{GlyphProps, WidgetGlyph}, svg_view::SvgViewComponent}},
	model::{cell_meta::CELL_TYPE_WIDGET_CONTAINER, widget::WidgetGlyphItem, widget_group::WidgetGroupListItemDto},
	store::cell::{CellInfoContext, NO_CONTEXT_FOUND},
	utils::{fetch, fetch_string, NULL_GLYPH, NULL_MODEL, NULL_UUID},
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

	let glyph_svg = use_state(|| IString::from(""));
	let widget_uuid = use_state( || value.uuid.clone());

	let model_handler = use_state(|| String::from(""));
	{
		let url = context.api_url.clone();
		let widget_uuid = widget_uuid.clone();
		let group_pk = value.group.clone();
		let group_name = group_name.clone();
		let glyph_svg = glyph_svg.clone();
		use_effect_with((*widget_uuid).clone(), move |uuid| {
			let uuid = uuid.clone();
			wasm_bindgen_futures::spawn_local(
				async move {
					// fetch group
					match fetch::<WidgetGroupListItemDto>(format!("{url}/widget-group/{group_pk}")).await {
						Ok(group) => group_name.set(group.name),
						Err(err) => log::error!("{err}"),
					}

					if uuid.eq(NULL_UUID) {
						let model = fetch_string(format!("{url}/widget/{NULL_UUID}/model")).await.unwrap_or(NULL_MODEL.to_owned());
						let glyph = fetch_string(format!("{url}/widget/{uuid}/glyph")).await.unwrap_or(NULL_GLYPH.to_owned());

						model_handler.set(model);
						glyph_svg.set(AttrValue::from(glyph));

						return;
					}

					// fetch model
					match fetch_string(format!("{url}/widget/{uuid}/model")).await {
						Ok(model) => model_handler.set(model),
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
        let glyph_svg = glyph_svg.clone();
		let widget_uuid = widget_uuid.clone();
        Callback::from(move |pk_glyph: (IString, IString)| {
			let (pk, glyph) = pk_glyph;
			widget_uuid.set(pk);
			glyph_svg.set(glyph);
		})
    };

	// let on_type_apply = {
	// 	let type_edit_mode = type_edit_mode.clone();
	// 	let widget_uuid = widget_uuid.clone();
	// 	let value = value.clone();
	// 	Callback::from(move |_: MouseEvent| {
	// 		// let mut new_value = value;
	// 		// new_value.uuid = (*widget_uuid).clone();
	// 		// value.set(new_value);

	// 		// type_edit_mode.set(false);     // togle type_edit_mode
	// 	})
	// };

	// // start apply process if true
	// let start_apply = use_selector(|state: &cell::State | state.start_apply);
	// {
	// 	// let on_detals_apply = on_detals_apply.clone();
	// 	let data_source = data_source.clone();
	// 	let my_value = my_value.clone();
	// 	use_effect_with(*start_apply, move |start| {
	// 		if *start {
	// 			let new_value = WidgetContainerXml {
	// 					ds: (*data_source).clone(),
	// 					..(*my_value).clone()
	// 				};

	// 			log::debug!("{new_value:?}");

	// 			let new_variant = DFlowVariant::WidgetContainer(new_value);
	// 			log::debug!("NEW WIDGET CONTAINER {:?}", new_variant);
	// 			// on_detals_apply.emit(new_variant);
	// 		}
	// 	})
	// };

    // let apply_ds = {
	// 	let data_source = data_source.clone();
	// 	Callback::from(move |ds: DataSourceXml| {
	// 		data_source.set(ds);
	// 	})
	// };


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

    // let type_editor_view = {
    //     // let edit_mode = edit_mode.clone();
	// 	let type_edit_mode = type_edit_mode.clone();
    //     if *type_edit_mode {
    //         html! { <button /*onclick={on_type_apply}*/><MdIcon icon={MdIconType::Check}/></button> }
    //         } else {
    //         html! { <button onclick={togle_type_edit}><MdIcon icon={MdIconType::Edit}/></button> }
    //         }
    // };

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

    let props_table = html! {
		<div>
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
		</div>
	};

    html! {
        <>
        <hr/>
        { props_table }
        <hr/>

		<div class="delim-label">{ "Тип объекта" }</div>

		<SvgViewComponent glyph={(*glyph_svg).clone()}/>
        <hr/>

		<div class="geSidebar" style="touch-action: none; display: block; transform-origin: left top;">
			{ widgets_view }
		</div>
        </>
    }
}