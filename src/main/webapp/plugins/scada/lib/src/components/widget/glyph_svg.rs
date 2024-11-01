use web_sys::Node;
use yew::prelude::*;
use implicit_clone::unsync::IString;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub svg: IString,
}

#[function_component(GlyphSvg)]
pub fn component(Props { svg }: &Props) -> Html {
    let div = use_memo(svg.clone(), |svg| {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
    
        let span = document.create_element("span").unwrap();   
        span.set_inner_html(svg);

        let node: Node = span.into();
        node
    });

    Html::VRef((*div).clone())
}