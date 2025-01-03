use common_model::utils::{css_to_map, map_to_css};
use wasm_bindgen::JsCast;
use web_sys::{Node, SvgElement};
use yew::prelude::*;
use implicit_clone::unsync::IString;

use crate::components::shared::{decode_glyph_to_svg, get_document};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub svg: IString,
}

#[function_component]
pub fn SvgViewComponent(Props {svg}: &Props) -> Html {
    let div = use_memo(svg.clone(), |svg| {
        // let svg = decode_glyph_to_svg(glyph.as_str());

        let document = get_document();

        let div = document.create_element("div").unwrap();
        div.set_class_name("svg-view");
        div.set_inner_html(&svg);

        if let Some(svg) = div.first_child() {
            if let Some(svg) = svg.dyn_into::<SvgElement>().ok() {
                if let Some(style) = svg.get_attribute("style") {
                    let mut style_map = css_to_map(style.as_str());
                    style_map.remove(&"display");
                    style_map.remove(&"width");
                    style_map.remove(&"height");

                    let style = map_to_css(style_map);
                    svg.set_attribute("style", &style).ok();
                };

                // svg.set_attribute("viewBox", "-0.5 -0.5 33 33").ok();
                svg.set_attribute("width", "128").ok();
                svg.set_attribute("height", "120").ok();
            };
        };

        Into::<Node>::into(div)
    });

    Html::VRef((*div).clone())
}