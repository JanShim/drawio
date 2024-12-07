use serde::{Deserialize, Serialize};
use yew::{function_component, html, Html, Properties};


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DFlowObject {
    id: i32,
    name: String,
    label: String,
    #[serde(rename="type")]
    is_type: String,
    has_children: bool,
}

// impl Into<Html> for DFlowObject {
//     fn into(self) -> Html {
//        html! {
//             <DFlowObjectComponent item={self}/>
//        }
//     }
// }

#[derive(PartialEq, Properties)]
pub struct DFlowObjectComponentProps {
    pub item: DFlowObject,
}

#[function_component(DFlowObjectComponent)]
pub fn dflow_object_component(props: &DFlowObjectComponentProps) -> Html {
    let DFlowObjectComponentProps { item } = props;
    html! {
        <p>{ serde_json::to_string(&item).unwrap() }</p>
    }
}