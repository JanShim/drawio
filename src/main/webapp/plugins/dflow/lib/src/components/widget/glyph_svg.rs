use web_sys::Node;
use yew::prelude::*;
use implicit_clone::unsync::IString;

use crate::components::shared::{decode_glyph_to_svg, get_document};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub svg: IString,
}

#[function_component]
pub fn GlyphSvg(Props { svg }: &Props) -> Html {
    let div = use_memo(svg.clone(), |svg| {
        let svg = decode_glyph_to_svg(svg.as_str());
        let document = get_document();

        let span = document.create_element("span").unwrap();
        span.set_inner_html(&svg);

        let node: Node = span.into();
        node
    });

    Html::VRef((*div).clone())
}