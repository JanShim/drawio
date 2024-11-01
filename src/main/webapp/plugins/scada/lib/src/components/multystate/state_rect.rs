use yew::prelude::*;
// use implicit_clone::unsync::IString;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub style: AttrValue,
}

#[function_component]
pub fn StateSampleRect(Props { style }: &Props) -> Html {

    // ========= view =================
    html! {
        <svg viewBox="0 0 40 20" width="40" height="20" xmlns="http://www.w3.org/2000/svg">
            <rect x="0" y="0" width="100%" height="100%" style={style}/>
        </svg>
    }
}