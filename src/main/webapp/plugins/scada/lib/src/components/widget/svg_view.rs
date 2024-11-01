use common_model::utils::{css_to_map, map_to_css};
use wasm_bindgen::JsCast;
use web_sys::{Node, SvgElement};
use yew::prelude::*;
use implicit_clone::unsync::IString;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub html: IString,
}

#[function_component(SvgViewComponent)]
pub fn component(props: &Props) -> Html {
    let div = use_memo(props.html.clone(), |html| {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
    
        let div = document.create_element("div").unwrap();   
        div.set_class_name("svg-view"); 
        div.set_inner_html(&html);

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

                svg.set_attribute("viewBox", "0 0 32 30").ok();
                svg.set_attribute("width", "128").ok();
                svg.set_attribute("height", "120").ok();
            };
        };

        let node: Node = div.into();
        node
    });

    Html::VRef((*div).clone())
}