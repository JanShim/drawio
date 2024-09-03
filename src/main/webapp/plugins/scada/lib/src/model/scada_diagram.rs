use std::{collections::HashMap, rc::Rc};

use serde::{Deserialize, Serialize};
use yew::{
    function_component, html, use_callback, use_state, Callback, Html, MouseEvent, Properties
};



#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
pub struct ScadaDiagramListDto {
    pub uuid: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ScadaDiagramDto {
    pub uuid: String,
    pub name: String,
    pub model: String,
}

impl ScadaDiagramDto {
    pub fn new(name: String, model: String) -> Self {
        ScadaDiagramDto {
            uuid: "00000000-0000-0000-0000-000000000000".to_owned(),
            name,
            model,
        }
    }
}


#[derive(PartialEq, Properties)]
pub struct Props {
    pub item: ScadaDiagramListDto,
    pub load: Callback<String>,
}

#[function_component(DiagramListItem)]
pub fn scada_diagram_component(props: &Props) -> Html {
    let Props {item, load} = props;
    let ScadaDiagramListDto {uuid, name} = item;

    let on_load = {
            let pk = uuid.clone();
            let load = load.clone();
            Callback::from(move |_: MouseEvent| {
                let pk = pk.clone();
                load.emit(pk);
            })
        };

    html! {
        <div>
            {format!("{} {}", uuid, name)} 
            <button onclick={on_load}>{ "load" }</button>
        </div>
    }
}


/*
   <object label="" id="0">
      <diagram item-type"schema" uuid="aaaaaaaaaa" />
    </object>
*/
#[derive(Serialize, Deserialize, Debug, PartialEq)]
// #[serde(rename_all = "camelCase")]
#[serde(rename = "object")]
pub struct DiagramMeta {
    #[serde(rename(serialize="@label"))]    
    label: String,
    #[serde(rename(serialize="@id"))]    
    id: String,
    // #[serde(flatten)]
    // data: HashMap<String, String>,
    // #[serde(rename(serialize="$value"))]    
    diagram: Diagram, 
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Diagram {
    #[serde(rename(serialize="@type", deserialize="type"))]    
    pub item_type: String,
    #[serde(rename(serialize="@uuid"))]    
    pub uuid: String,
}


// ==========================================================
#[cfg(test)]
mod tests {
    // use std::collections::HashMap;

    use super::*;

    // #[derive(Serialize, Deserialize, Debug)]
    // #[serde(rename = "object")]
    // // #[serde(rename_all = "camelCase")]    
    // struct HashObject {
    //     // #[serde(serialize_with = "serialize_struct_attr")]
    //     // #[serde(rename(serialize="@uuid"))]    
    //     // uuid: String,
    //     #[serde(flatten)]
    //     data: HashMap<String, String>,
    // }

    // impl HashObject {
    //     pub fn new() -> Self {
    //         Self {
    //             // uuid: "aaaaaaaaaa".to_owned(),
    //             data: {
    //                 let mut m = HashMap::<String, String>::new();
    //                 m.insert("type".to_owned(), "schema".to_owned());
    //                 m
    //             }
    //         }
    //     }
        
    // }
    

    #[test]
    fn xml_diagram_meta_deser_works() {
        let xml = r#"<object label="" id="0">
      <diagram type="schema" uuid="aaaaaaaaaa" />
    </object>"#;

        let diagram = serde_xml_rs::from_str::<DiagramMeta>(xml);    
        match diagram {
            Ok(item) => {
                println!("{item:#?}");
                assert_eq!(item.id, "0".to_owned());
                assert_eq!(item.diagram.uuid, "aaaaaaaaaa".to_owned());

            },
            Err(err) => panic!("err: {}", err),
        }
    }

    #[test]
    fn xml_diagram_meta_ser_works() {
        let item = DiagramMeta {
            id: "0".to_owned(),
            label: "".to_owned(),
            diagram: Diagram {
                item_type: "schema".to_owned(),
                uuid: "aaaaaaaaaa".to_owned(),
            }
        };

        let str = serde_xml_rs::to_string(&item).unwrap();
        println!("{str}");        

        let diagram = serde_xml_rs::from_str::<DiagramMeta>(&str).unwrap();    

        assert_eq!(item, diagram);
    }

    #[test]
    fn xml_diagram_deser_works() {
        let xml = r#"<diagram type="schema" uuid="aaaaaaaaaa" />"#;

        let diagram = serde_xml_rs::from_str::<Diagram>(xml);    
        match diagram {
            Ok(item) => {
                assert_eq!(item.item_type, "schema".to_owned());
                assert_eq!(item.uuid, "aaaaaaaaaa".to_owned());
            },
            Err(err) => panic!("err: {}", err),
        }
    }    

    // #[test]
    // fn xml_hashmap_deser_works() {
    //     let xml = r#"<diagram type="schema" uuid="aaaaaaaaaa" />"#;

    //     let diagram = serde_xml_rs::from_str::<HashObject>(xml);    
    //     match diagram {
    //         Ok(item) => {
    //             println!("{item:#?}");

    //             assert_eq!(item.uuid, "aaaaaaaaaa");
    //             assert_eq!(item.data.get("type"), Some("schema".to_owned()).as_ref());
    //         },
    //         Err(err) => panic!("err: {}", err),
    //     }
    // }    

    // #[test]
    // fn xml_hashmap_ser_works() {
    //     // let xml = r#"<diagram type="schema" uuid="aaaaaaaaaa" />"#;

    //     let item = HashObject::new();
    //     let str = serde_xml_rs::to_string(&item).unwrap();

    //     // serde_xml_rs::Serializer.

    //     println!("{str}");

    //     // let diagram = serde_xml_rs::from_str::<HashObject>(xml);    
    //     // match diagram {
    //     //     Ok(item) => {
    //     //         println!("{item:#?}");

    //     //         assert_eq!(item.uuid, "aaaaaaaaaa");
    //     //         assert_eq!(item.data.get("type"), Some("schema".to_owned()).as_ref());
    //     //     },
    //     //     Err(err) => panic!("err: {}", err),
    //     // }
    // }    


}