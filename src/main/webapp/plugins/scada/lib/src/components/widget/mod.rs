use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlAnchorElement;
use data_source::DataSourceComponent;
use yewdux::{use_selector, use_store};
use implicit_clone::unsync::IString;
use svg_view::SvgViewComponent;

use crate::{errors::CellStateError, model::cell_meta::{widget::{self, WidgetMeta}, CellMetaVariant}, store::cell};

pub mod data_source;
pub mod svg_view;
pub mod list_item;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub edit_mode: bool,
}

#[function_component(WidgetComponent)]
pub fn component(Props { edit_mode }: &Props) -> Html {
    // let (_, cell_store_dispatch) = use_store::<cell::CellState>();
    let widget = use_selector(|cell_state: &cell::CellState| {
		if let CellMetaVariant::Widget(widget) = cell_state.meta.data.clone() {
			return widget;
		};
		log::error!("{}", CellStateError::NotWidget);
		WidgetMeta::default()
	});  

    let is_edit = use_state(|| false);
    let togle_edit = {
        let edit = is_edit.clone();
        Callback::from(move |_: MouseEvent| { edit.set(!*edit); })
    };  


    let widget_state = use_reducer(|| (*widget).clone());

    let inner_svg = use_state(|| IString::from("<span>???</span>"));

    let on_item_select = {
        let widget_state = widget_state.clone();
        let inner_svg = inner_svg.clone();
        Callback::from(move |e: MouseEvent| {
            e.target().and_then(|t| t.dyn_into::<HtmlAnchorElement>().ok())
                    .map(|elem| {
                        if let Some(id) = elem.get_attribute("id") {
                            widget_state.dispatch(widget::Action::SetUuid(id.into()));
                        }

                        inner_svg.set(elem.inner_html().into());
                    });
        })
    };

	let togle_apply = {
		let is_edit = is_edit.clone();
		Callback::from(move |_: MouseEvent| {

			is_edit.set(!*is_edit);     // togle is_edit
		})
	};

    // ------------ View Items
    let data_source_view = {
        let props = yew::props!(data_source::Props {
            ds: widget.data_source.clone(),
            edit_mode: *edit_mode,
        });
        html! {<DataSourceComponent ..props/>}
    };    

    let svg_view = {
        let inner_svg = inner_svg.clone();
        let props = yew::props!(svg_view::Props {
            html: (*inner_svg).clone(),
        });
        html! {<SvgViewComponent ..props/>}
    };

    let img_view = {
        let is_edit = is_edit.clone();
        if *edit_mode {
            if *is_edit { 
                html! { <img class="img-16" src="images/checkmark.gif" onclick={togle_apply}/>  }
             } else {
                html! { <img class="img-16" src="images/edit16.png" onclick={togle_edit}/> }
             }
        } else {
            html! { <span/> }
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
		<a id="00000000-0000-0000-0000-000000000001" onclick={on_item_select.clone()} class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<path d="M 1.5 6.3 L 16 15 L 1.5 23.7 Z M 30.5 6.3 L 16 15 L 30.5 23.7 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" onclick={on_item_select.clone()} class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<path d="M 1.5 6.3 L 16 15 L 1.5 23.7 Z M 30.5 6.3 L 16 15 L 30.5 23.7 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" onclick={on_item_select.clone()} class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<ellipse cx="16" cy="15" rx="5.8" ry="5.568" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</ellipse>
							<path d="M 1.5 6.3 L 16 15 L 1.5 23.7 Z M 30.5 6.3 L 16 15 L 30.5 23.7 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<ellipse cx="16" cy="15" rx="5.8" ry="5.568" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</ellipse>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" onclick={on_item_select.clone()} class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<ellipse cx="16" cy="15" rx="5.8" ry="5.568" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</ellipse>
							<path d="M 1.5 6.3 L 16 15 L 1.5 23.7 Z M 30.5 6.3 L 16 15 L 30.5 23.7 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<ellipse cx="16" cy="15" rx="5.8" ry="5.568" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</ellipse>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<ellipse cx="16" cy="15" rx="5.8" ry="5.568" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</ellipse>
							<path d="M 1.5 6.3 L 16 15 L 1.5 23.7 Z M 30.5 6.3 L 16 15 L 30.5 23.7 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<ellipse cx="16" cy="15" rx="5.8" ry="5.568" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</ellipse>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<path d="M 1.5 6.3 L 1.5 23.7 M 30.5 6.3 L 30.5 23.7 M 2.95 7.17 L 29.05 22.83" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<ellipse cx="16" cy="14.91" rx="2.9" ry="2.871" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</ellipse>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<path d="M 1.5 6.3 L 1.5 23.7 M 30.5 6.3 L 30.5 23.7 M 2.95 7.17 L 29.05 22.83" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 27.38 20.48 L 29.25 22.92 L 26.15 22.45 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<path d="M 13.1 15 L 16 10.65 L 18.9 15 L 16 19.35 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 1.5 6.3 L 16 15 L 1.5 23.7 Z M 30.5 6.3 L 16 15 L 30.5 23.7 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 13.1 15 L 16 10.65 L 18.9 15 L 16 19.35 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<path d="M 14.55 8.04 L 17.45 8.04 L 16 23.7 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 1.5 6.3 L 16 15 L 1.5 23.7 Z M 30.5 6.3 L 16 15 L 30.5 23.7 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 14.55 8.04 L 17.45 8.04 L 16 23.7 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<path d="M 16 15 L 16 23.18" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 16 15 L 16 23.18" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 15.42 22.67 L 16.58 22.67 L 16 23.53 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 1.5 6.3 L 16 15 L 1.5 23.7 Z M 30.5 6.3 L 16 15 L 30.5 23.7 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 16 15 L 16 23.18" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 16 15 L 16 23.18" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 15.42 22.67 L 16.58 22.67 L 16 23.53 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000002" onclick={on_item_select.clone()} class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<path d="M 9.25 5.55 L 22.75 5.55 M 16 5.55 L 16 20.4" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 9.25 5.55 L 22.75 5.55 M 16 5.55 L 16 20.4" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 2.5 12.3 L 16 20.4 L 2.5 28.5 Z M 29.5 12.3 L 16 20.4 L 29.5 28.5 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<path d="M 16 7.44 L 16 20.4" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 16 7.44 L 16 20.4" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 9.25 7.44 C 10.75 5.63 13.29 4.54 16 4.54 C 18.71 4.54 21.25 5.63 22.75 7.44 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 2.5 12.3 L 16 20.4 L 2.5 28.5 Z M 29.5 12.3 L 16 20.4 L 29.5 28.5 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<ellipse cx="16" cy="6.63" rx="6.75" ry="2.43" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</ellipse>
							<path d="M 16 9.06 L 16 20.4" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 16 9.06 L 16 20.4" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 2.5 12.3 L 16 20.4 L 2.5 28.5 Z M 29.5 12.3 L 16 20.4 L 29.5 28.5 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 9.25 6.63 L 22.75 6.63" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 9.25 6.63 L 22.75 6.63" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<rect x="11.27" y="1.5" width="9.45" height="9.45" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</rect>
							<path d="M 16 10.95 L 16 20.4" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 16 10.95 L 16 20.4" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<g fill="#000000" font-family="&quot;Helvetica&quot;" font-weight="bold" text-anchor="middle" font-size="3.7800000000000002px" style="pointer-events: none;">
							</g>
							<path d="M 2.5 12.3 L 16 20.4 L 2.5 28.5 Z M 29.5 12.3 L 16 20.4 L 29.5 28.5 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<rect x="11.27" y="1.5" width="9.45" height="9.45" fill="none" stroke="white" visibility="hidden" stroke-width="9" style="pointer-events: none;">
							</rect>
							<rect x="11.27" y="1.5" width="9.45" height="9.45" fill="none" stroke="none" style="pointer-events: none;">
							</rect>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<rect x="11.27" y="1.5" width="9.45" height="9.45" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</rect>
							<path d="M 16 10.95 L 16 20.4" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 16 10.95 L 16 20.4" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<g fill="#000000" font-family="&quot;Helvetica&quot;" font-weight="bold" text-anchor="middle" font-size="3.7800000000000002px" style="pointer-events: none;">
								<text x="16" y="7.84" style="pointer-events: none;">
									{"D"}
								</text>
							</g>
							<path d="M 2.5 12.3 L 16 20.4 L 2.5 28.5 Z M 29.5 12.3 L 16 20.4 L 29.5 28.5 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<ellipse cx="16" cy="6.22" rx="4.7250000000000005" ry="4.7250000000000005" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</ellipse>
							<path d="M 16 10.95 L 16 20.4" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 16 10.95 L 16 20.4" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<g fill="#000000" font-family="&quot;Helvetica&quot;" font-weight="bold" text-anchor="middle" font-size="3.7800000000000002px" style="pointer-events: none;">
								<text x="16" y="7.84" style="pointer-events: none;">
									{"E/H"}
								</text>
							</g>
							<path d="M 2.5 12.3 L 16 20.4 L 2.5 28.5 Z M 29.5 12.3 L 16 20.4 L 29.5 28.5 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<rect x="11.27" y="1.5" width="9.45" height="9.45" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</rect>
							<path d="M 16 10.95 L 16 20.4" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 16 10.95 L 16 20.4" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<g fill="#000000" font-family="&quot;Helvetica&quot;" font-weight="bold" text-anchor="middle" font-size="3.7800000000000002px" style="pointer-events: none;">
								<text x="16" y="7.84" style="pointer-events: none;">
									{"K"}
								</text>
							</g>
							<path d="M 2.5 12.3 L 16 20.4 L 2.5 28.5 Z M 29.5 12.3 L 16 20.4 L 29.5 28.5 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<ellipse cx="16" cy="6.22" rx="4.7250000000000005" ry="4.7250000000000005" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</ellipse>
							<path d="M 16 10.95 L 16 20.4" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 16 10.95 L 16 20.4" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<g fill="#000000" font-family="&quot;Helvetica&quot;" font-weight="bold" text-anchor="middle" font-size="3.7800000000000002px" style="pointer-events: none;">
								<text x="16" y="7.84" style="pointer-events: none;">
									{"M"}
								</text>
							</g>
							<path d="M 2.5 12.3 L 16 20.4 L 2.5 28.5 Z M 29.5 12.3 L 16 20.4 L 29.5 28.5 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<rect x="11.27" y="1.5" width="9.45" height="9.45" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</rect>
							<path d="M 16 10.95 L 16 20.4" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 16 10.95 L 16 20.4" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<g fill="#000000" font-family="&quot;Helvetica&quot;" font-weight="bold" text-anchor="middle" font-size="3.7800000000000002px" style="pointer-events: none;">
								<text x="16" y="7.84" style="pointer-events: none;">
									{"P"}
								</text>
							</g>
							<path d="M 2.5 12.3 L 16 20.4 L 2.5 28.5 Z M 29.5 12.3 L 16 20.4 L 29.5 28.5 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<rect x="11.27" y="1.5" width="9.45" height="9.45" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</rect>
							<path d="M 16 10.95 L 16 20.4" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 16 10.95 L 16 20.4" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<g fill="#000000" font-family="&quot;Helvetica&quot;" font-weight="bold" text-anchor="middle" font-size="3.7800000000000002px" style="pointer-events: none;">
								<text x="16" y="7.84" style="pointer-events: none;">
									{"S"}
								</text>
							</g>
							<path d="M 2.5 12.3 L 16 20.4 L 2.5 28.5 Z M 29.5 12.3 L 16 20.4 L 29.5 28.5 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<rect x="11.27" y="1.5" width="9.47" height="8.69" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</rect>
							<path d="M 19.97 12.84 L 23.38 10.95 L 26.8 12.84 L 23.38 14.73 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 16.01 10.19 L 16.01 20.4 M 16.01 12.84 L 19.97 12.84" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 16.01 10.19 L 16.01 20.4 M 16.01 12.84 L 19.97 12.84" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<g fill="#000000" font-family="&quot;Helvetica&quot;" font-weight="bold" text-anchor="middle" font-size="6.210000000000001px" style="pointer-events: none;">
								<text x="16.01" y="8.68" style="pointer-events: none;">
									{"S"}
								</text>
							</g>
							<g fill="#000000" font-family="&quot;Helvetica&quot;" text-anchor="middle" font-size="2.32875px" style="pointer-events: none;">
								<text x="23.38" y="13.73" style="pointer-events: none;">
									{"R"}
								</text>
							</g>
							<path d="M 2.5 12.3 L 16 20.4 L 2.5 28.5 Z M 29.5 12.3 L 16 20.4 L 29.5 28.5 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<path d="M 16 1.5 L 16 20.4 M 14.64 4.52 L 17.36 3.01 M 13.81 7.55 L 18.19 5.28 M 12.98 11.33 L 19.02 8.3 M 12.22 15.11 L 19.78 10.95" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 16 1.5 L 16 20.4 M 14.64 4.52 L 17.36 3.01 M 13.81 7.55 L 18.19 5.28 M 12.98 11.33 L 19.02 8.3 M 12.22 15.11 L 19.78 10.95" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 2.5 12.3 L 16 20.4 L 2.5 28.5 Z M 29.5 12.3 L 16 20.4 L 29.5 28.5 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<rect x="11.27" y="1.5" width="9.45" height="9.45" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</rect>
							<path d="M 16 10.95 L 16 20.4" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 16 10.95 L 16 20.4" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<g fill="#000000" font-family="&quot;Helvetica&quot;" font-weight="bold" text-anchor="middle" font-size="3.7800000000000002px" style="pointer-events: none;">
								<text x="16" y="7.84" style="pointer-events: none;">
									{"W"}
								</text>
							</g>
							<path d="M 2.5 12.3 L 16 20.4 L 2.5 28.5 Z M 29.5 12.3 L 16 20.4 L 29.5 28.5 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<rect x="11.95" y="1.5" width="8.07" height="8.69" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</rect>
							<path d="M 15.99 10.19 L 15.99 20.4 M 20.02 5.85 L 29.5 5.85 M 25.46 4.33 L 24.06 7.36 M 26.34 4.33 L 24.94 7.36" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 15.99 10.19 L 15.99 20.4 M 20.02 5.85 L 29.5 5.85 M 25.46 4.33 L 24.06 7.36 M 26.34 4.33 L 24.94 7.36" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 2.5 12.3 L 16 20.4 L 2.5 28.5 Z M 29.5 12.3 L 16 20.4 L 29.5 28.5 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 15.99 5.85 L 15.99 10.19 M 11.95 5.85 L 20.02 5.85" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 15.99 5.85 L 15.99 10.19 M 11.95 5.85 L 20.02 5.85" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<rect x="11.95" y="1.5" width="8.07" height="8.69" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</rect>
							<path d="M 15.99 10.19 L 15.99 20.4 M 20.02 3.67 L 29.5 3.67 M 25.46 2.16 L 24.06 5.19 M 26.34 2.16 L 24.94 5.19 M 20.02 8.02 L 29.5 8.02 M 25.46 6.51 L 24.06 9.53 M 26.34 6.51 L 24.94 9.53" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 15.99 10.19 L 15.99 20.4 M 20.02 3.67 L 29.5 3.67 M 25.46 2.16 L 24.06 5.19 M 26.34 2.16 L 24.94 5.19 M 20.02 8.02 L 29.5 8.02 M 25.46 6.51 L 24.06 9.53 M 26.34 6.51 L 24.94 9.53" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 2.5 12.3 L 16 20.4 L 2.5 28.5 Z M 29.5 12.3 L 16 20.4 L 29.5 28.5 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 15.99 5.85 L 15.99 10.19 M 11.95 5.85 L 20.02 5.85" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 15.99 5.85 L 15.99 10.19 M 11.95 5.85 L 20.02 5.85" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<path d="M 19.67 6.9 L 26.8 12.37 M 16 20.4 L 23.18 9.67" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 19.67 6.9 L 26.8 12.37 M 16 20.4 L 23.18 9.67" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 2.5 12.3 L 16 20.4 L 2.5 28.5 Z M 29.5 12.3 L 16 20.4 L 29.5 28.5 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<path d="M 5.5 1.35 L 16 7.63 L 5.5 13.91 Z M 16 7.63 L 26.5 1.35 L 26.5 13.91 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 16 7.63 L 16 15" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 16 7.63 L 16 15" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 11.8 15 L 20.2 15 L 16 21.82 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 11.8 28.65 L 16 21.82 L 20.2 28.65" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<path d="M 12 3.8 L 20 3.8 M 16 3.8 L 16 12.58" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 12 3.8 L 20 3.8 M 16 3.8 L 16 12.58" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 8 7.8 L 16 12.58 L 8 17.37 Z M 16 12.58 L 24 7.8 L 24 17.37 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 16 12.58 L 16 18.2" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 16 12.58 L 16 18.2" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 12.8 18.2 L 19.2 18.2 L 16 23.4 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 12.8 28.6 L 16 23.4 L 19.2 28.6" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<path d="M 16 12.1 L 30.5 3.4 L 30.5 20.8 Z M 16 12.1 L 24.7 26.6 L 7.3 26.6 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<path d="M 10.25 4.65 L 21.75 4.65 M 16 4.65 L 16 17.3" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 10.25 4.65 L 21.75 4.65 M 16 4.65 L 16 17.3" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 16 17.3 L 27.5 10.4 L 27.5 24.2 Z M 16 17.3 L 22.9 28.8 L 9.1 28.8 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<ellipse cx="16" cy="12.1" rx="4.64" ry="4.64" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</ellipse>
							<path d="M 16 12.1 L 30.5 3.4 L 30.5 20.8 Z M 16 12.1 L 24.7 26.6 L 7.3 26.6 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<ellipse cx="16" cy="12.1" rx="4.64" ry="4.64" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</ellipse>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<path d="M 10.25 4.65 L 21.75 4.65 M 16 4.65 L 16 17.3" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 10.25 4.65 L 21.75 4.65 M 16 4.65 L 16 17.3" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<ellipse cx="16" cy="17.3" rx="3.68" ry="3.6801840000000006" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</ellipse>
							<path d="M 16 17.3 L 27.5 10.4 L 27.5 24.2 Z M 16 17.3 L 22.9 28.8 L 9.1 28.8 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<ellipse cx="16" cy="17.3" rx="3.68" ry="3.68046" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</ellipse>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<path d="M 1.5 3.4 L 16 12.1 L 1.5 20.8 Z M 30.5 3.4 L 16 12.1 L 30.5 20.8 Z M 16 12.1 L 24.7 26.6 L 7.3 26.6 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<path d="M 10.25 4.65 L 21.75 4.65 M 16 4.65 L 16 17.3" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 10.25 4.65 L 21.75 4.65 M 16 4.65 L 16 17.3" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 4.5 10.4 L 16 17.3 L 4.5 24.2 Z M 27.5 10.4 L 16 17.3 L 27.5 24.2 Z M 16 17.3 L 22.9 28.8 L 9.1 28.8 Z" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
		<a id="00000000-0000-0000-0000-000000000001" class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
			<svg style="left: 1px; top: 1px; width: 32px; height: 30px; display: block; position: relative; overflow: hidden; pointer-events: none;">
				<g style="pointer-events: none;">
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
						<g transform="translate(0.5,0.5)" style="visibility: visible; pointer-events: none;">
							<rect x="1.5" y="6.3" width="29" height="17.4" fill="rgb(241, 243, 244)" stroke="rgb(0, 0, 0)" stroke-width="1.3" style="pointer-events: none;">
							</rect>
							<path d="M 3.82 7.69 L 3.82 22.31 M 28.18 7.69 L 28.18 22.31 M 4.98 8.42 L 26.84 21.68 M 16 6.3 L 17.45 7.17 L 14.55 8.91 L 17.45 10.65 L 14.55 12.39 L 17.45 14.13 L 15.71 15" fill="none" stroke="white" stroke-width="9.3" stroke-linejoin="round" stroke-miterlimit="10" visibility="hidden" style="pointer-events: none;">
							</path>
							<path d="M 3.82 7.69 L 3.82 22.31 M 28.18 7.69 L 28.18 22.31 M 4.98 8.42 L 26.84 21.68 M 16 6.3 L 17.45 7.17 L 14.55 8.91 L 17.45 10.65 L 14.55 12.39 L 17.45 14.13 L 15.71 15" fill="none" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
							<path d="M 25.45 19.69 L 27.01 21.75 L 24.41 21.35 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-width="1.3" stroke-linejoin="round" stroke-miterlimit="10" style="pointer-events: none;">
							</path>
						</g>
					</g>
					<g style="pointer-events: none;">
					</g>
					<g style="pointer-events: none;">
					</g>
				</g>
			</svg>
		</a>
	</div>
</div>

        
        </>
    }
}    