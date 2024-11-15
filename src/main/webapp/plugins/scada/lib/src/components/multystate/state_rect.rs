use yew::prelude::*;
// use implicit_clone::unsync::IString;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub style: AttrValue,
    pub text_style: AttrValue,
}

#[function_component]
pub fn StateSampleRect(Props {style, text_style }: &Props) -> Html {
    html! {
        <svg viewBox="0 0 60 30" width="40" height="20" xmlns="http://www.w3.org/2000/svg">
        <g style={style}>
            <g>
                <rect x="0" y="0" width="60" height="30" stroke="none"/>
            </g>
            <g>
                <g font-family="&quot;Helvetica&quot;" font-weight="bold" text-anchor="middle" font-size="12px">
                    <text x="29.5" y="19.5" style={text_style}>{"text"}</text>
                </g>
            </g>
        </g>
        </svg>
    }
}