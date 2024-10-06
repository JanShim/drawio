use yew::prelude::*;
use implicit_clone::unsync::IString;

use crate::components::widget::glyph_svg::GlyphSvg;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub pk: AttrValue,
    pub on_select: Callback<(IString, IString)>,
    pub glyph: IString,
}

#[function_component(WidgetGlyph)]
pub fn component(Props {pk, on_select, glyph }: &Props) -> Html {

    let on_click = {
        let on_select = on_select.clone();
        let pk = pk.clone();
        let glyph = glyph.clone();
        Callback::from(move |_: MouseEvent| {
            on_select.emit((pk.to_string().into(), glyph.clone()))
        })
    };

    html! {
		<a id={pk} onclick={on_click} class="geItem" style="overflow: hidden; width: 34px; height: 32px; padding: 1px;">
            <GlyphSvg svg={glyph.clone()}/>
		</a>        
    }
}