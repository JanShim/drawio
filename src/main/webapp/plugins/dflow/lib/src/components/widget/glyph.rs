use yew::prelude::*;
use implicit_clone::unsync::IString;

use crate::components::widget::glyph_svg::GlyphSvg;

#[derive(Properties, PartialEq, Debug)]
pub struct GlyphProps {
    pub pk: AttrValue,
    pub name: AttrValue,
    pub name_ru: Option<AttrValue>,
    pub glyph: IString,
    pub on_select: Callback<(IString, IString)>,
}

#[function_component]
pub fn WidgetGlyph(GlyphProps {
    pk,
    name,
    name_ru ,
    glyph,
    on_select,
}: &GlyphProps) -> Html {

    let on_click = {
        let on_select = on_select.clone();
        let pk = pk.clone();
        let glyph = glyph.clone();
        Callback::from(move |_: MouseEvent| {
            on_select.emit((pk.to_string().into(), glyph.clone()))
        })
    };

    html! {
		<a id={pk} onclick={on_click} class="widgetGlyphItem">
            // <GlyphSvg svg={glyph.clone()}/>
            <img src={ format!("data:image/svg+xml;base64,{}", glyph) } width="36" height="36" title={ name_ru.clone().unwrap_or((*name).clone()) }/>
		</a>
    }
}