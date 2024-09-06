use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlDivElement;

use crate::model::mx_cell::MxCell;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub val: String,
    pub cell: MxCell,
}

#[function_component(App)]
pub fn app(props: &Props) -> Html {

    // let node = use_memo(
    //     (),
    //     |_| {
    //         let window = web_sys::window().expect("no global `window` exists");
    //         let document = window.document().expect("should have a document on window");
    //         let elem = document.get_element_by_id("container").expect("must be id=container");

    //         // Create a div element from the document
    //         // let div: Element = document.create_element("div").unwrap();
    //         // Add content, classes etc.
    //         elem.set_inner_html("Hello, World!");
    //         // Convert Element into a Node
    //         let node = elem.into();
    //         // Return that Node as a Html value
    //         Html::VRef(node)
    //     },
    // );

    // // use_memo return Rc so we need to deref and clone
    // (*node).clone()

    let counter = use_state(|| 0);
    let up = {
        let counter = counter.clone();
        Callback::from(move |_: MouseEvent| {
            counter.set(*counter + 1);
        })
    };
    let dwn = {
        let counter = counter.clone();
        Callback::from(move |_: MouseEvent| {
            counter.set(*counter - 1);
        })
    };

    if let Ok(el) = props.cell.get_value() {
        // if let Some(style) = props.cell.mx_style() {
        //     el.set_attribute("style", style.as_str()).ok();
        // }

        // let ch = el.children();
        // for i in 0..ch.length() {
        //     if let Some(e) = ch.item(i) {
        //         e.set_attribute("new-attr", "new value").ok();
        //         log::info!("cell attributes: {:?}", e.get_attribute_names());
        //     }
        // }
    }

    // let up = Callback::from(move |e: Event| {
    //     // let target: EventTarget = e
    //     //     .target()
    //     //     .expect("Event should have a target when dispatched");
    //     // // You must KNOW target is a HtmlInputElement, otherwise
    //     // // the call to value would be Undefined Behaviour (UB).
    //     // // Here we are sure that this is input element so we can convert it to the appropriate type without checking
    //     // input_value_handle.set(target.unchecked_into::<HtmlInputElement>().value());
    //     let counter = counter.clone();
    //     let value = *counter + 1;
    //     counter.set(value);
    // });    

    html! {
        <div>
            <p>{&props.val}</p>
            <button onclick={up}>{ "+1" }</button><button onclick={dwn}>{ "-1" }</button>
            <p>{ *counter }</p>
        </div>
    }    

    // html! {
    //     <main>
    //         <h1>{ "Hello World!" }</h1>
    //         <span class="subtitle">{ "from Yew with! " }<i class="heart" /></span>
    //     </main>
    // }
}


#[wasm_bindgen(js_name=renderCell)]
pub fn render_cell(div: HtmlDivElement, cell: MxCell) {
    let props  = Props {
        val: "CELL".to_owned(),
        cell,
    };
    yew::Renderer::<App>::with_root_and_props(div.into(), props).render();

    // if let Ok(el) = cell.value() {
    //     let ch = el.children();
    //     for i in 0..ch.length() {
    //         if let Some(e) = ch.item(i) {
    //             e.set_attribute("new-attr", "new value").ok();
    //             log::info!("cell attributes: {:?}", e.get_attribute_names());
    //         }
    //     }
    // }

}
