use serde::{Deserialize, Serialize};
use yew::{function_component, html, Html, Properties};


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScadaObject {
    id: i32,
    name: String,
    label: String,
    #[serde(rename="type")]
    is_type: String,
    has_children: bool,
}

// impl Into<Html> for ScadaObject {
//     fn into(self) -> Html {
//        html! {
//             <ScadaObjectComponent item={self}/>
//        }
//     }
// }

#[derive(PartialEq, Properties)]
pub struct ScadaObjectComponentProps {
    pub item: ScadaObject,
}

#[function_component(ScadaObjectComponent)]
pub fn scada_object_component(props: &ScadaObjectComponentProps) -> Html {
    let ScadaObjectComponentProps { item } = props;
    html! {
        <p>{ serde_json::to_string(&item).unwrap() }</p>
    }
}