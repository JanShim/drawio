use std::{cell::RefCell, hash::{self, Hash}, rc::Rc};

use futures_signals::signal::Mutable;
use wasm_bindgen::prelude::*;
use dominator::{attrs, class, clone, events, html, styles, svg, with_node, Dom};
use js_sys::wasm_bindgen;
use web_sys::HtmlDivElement;

use crate::{mx_graph::{
    mx_cell::{AppCell, MxCell}, 
    mx_editor::MxEditor, 
}, utils::compare_eq_options};

pub struct App {
    editor: MxEditor,
    div:  HtmlDivElement,
    cell: Mutable<Option<AppCell>>,
}

impl App {
    pub fn new(editor: MxEditor, div: HtmlDivElement) -> Self {
        let me = Self {
            editor,
            div,
            cell: Mutable::new(None),
        };

        dominator::append_dom(&me.div, me.render());

        log::debug!("SCADA started");
        me
    }

    pub fn set_cell(&self, cell: Option<AppCell>) {
        self.cell.set_if(cell, |a, b| {
            !compare_eq_options(a, b)
        });
    }

    pub fn render(&self) -> Dom {
        // // // Define CSS styles
        // // static ROOT_CLASS: Lazy<String> = Lazy::new(|| class! {
        // //     .style("display", "inline-block")
        // //     .style("background-color", "black")
        // //     .style("padding", "10px")
        // // });

        // // static TEXT_CLASS: Lazy<String> = Lazy::new(|| class! {
        // //     .style("color", "white")
        // //     .style("font-weight", "bold")
        // // });

        // let ccc = svg! ("g", {
        //     .attrs! {
        //         fill: "white", 
        //         stroke: "green", 
        //         "stroke-width": "5",
        //         // transform: "translate(10 -10)"
        //     }
        //     .children(&mut [
        //         svg!("circle", {.attrs! {cx: "40", cy: "40", r: "10"}}),
        //         svg!("circle", {.attrs! {cx: "60", cy: "60", r: "10"}}),
        //     ])
        // });

        // // Create the DOM nodes
        // svg!("svg", {
        //     .attrs! {
        //         viewBox: "0 0 240 80",
        //         // xmlns: "http://www.w3.org/2000/svg",
        //     }
        //     // .class(&*ROOT_CLASS)
        //     .children(&mut [
        //         // html!("div", {
        //         //     .class(&*TEXT_CLASS)
        //         //     .text_signal(state.counter.signal().map(|x| format!("Counter: {}", x)))
        //         // }),
        //         svg! ("text", {
        //             .attrs! { x: "20", y: "35", class: "small", }
        //             .text("My")
        //         }),
        //         svg! ("text", {
        //             .attrs! { x: "20", y: "55", class: "small", }
        //             .class("mouse_hover")
        //             .text_signal(state.counter.signal().map(|x| format!("{}", x)))
        //             .event(clone!(state => move |_: events::Click| {
        //                 state.counter.replace_with(|x| *x + 1);
        //             }))
        //             .with_node!(el => {
        //                 .event(move |_: events::MouseEnter| {
        //                     debug!("{}", el.outer_html());
        //                 })
        //             })
        //         }),                
        //         svg! ("text", {
        //             .attrs! { x: "40", y: "35", class: "heavy", }
        //             .text("cat")
        //         }),
        //         svg! ("text", {
        //             .attrs! { x: "55", y: "55", class: "small", }
        //             .text("is")
        //         }),
        //         svg! ("text", {
        //             .attrs! { x: "65", y: "55", class: "Rrrrr", 
        //             transform: "rotate(45, 100, 10)"
        //             }
        //             .text("Grumpy!")
        //         }),
        //         ccc,

        //     ])
        // })


        // Create the DOM nodes
        html!("div" => web_sys::HtmlDivElement, {
            .styles!{color: "red"}
            .child_signal(
                self.cell.signal_ref(|c| {
                    match c {
                        Some(cell) => Some(cell.render()),
                        None => Some(AppCell::clean())
                    }
                })
            )
        })


    }
}


#[wasm_bindgen]
pub struct AppApi {
    application: App,
}

#[wasm_bindgen]
impl AppApi {
    #[wasm_bindgen(constructor)]
    pub fn new(editor: MxEditor, div: HtmlDivElement) -> Self {
        Self {
            application: App::new(editor, div),
        }
    }

    #[wasm_bindgen]
    pub fn cell_clicked(&self, cell: MxCell) {
        if cell.is_null() {
            self.application.set_cell(None);
        } else {
            log::debug!("cell clicked-> {:?}", cell);
            self.application.set_cell(Some(AppCell::new(cell)));
        }
    }

    #[wasm_bindgen]
    pub fn cell_updated(&self, cell: MxCell) {
        if cell.is_null() {
            log::error!("can't update null cell");
        } else {
            log::debug!("cell updated-> {:?}", cell);
            let lock = self.application.cell.lock_ref();
            if let Some(cell) = &*lock {
                cell.updated();
            } 
        }
    }    
}

//========================== TESTS ===========================================
#[cfg(test)]
mod tests {
    use futures::{join, FutureExt};
    use futures_signals::{
        map_ref, signal::SignalExt, signal_map::{MapDiff::*, MutableBTreeMap, SignalMapExt}
    };
    use std::{collections::{BTreeMap, HashMap}, time::Duration};
    use tokio::time;

    use crate::utils::calculate_hash;

    use super::*;

    pub struct App {
        cell_mutable: Mutable<Option<i32>>,
    }

    impl App {
        pub fn new() -> Self {
            Self {
                cell_mutable: Mutable::new(None),
            }
        }
    
        pub fn set_cell(&self, cell: Option<i32>) {
            self.cell_mutable.set_if(cell, |a, b| {
                if a.is_none() && b.is_none() {
                    return false
                }
                if let Some(a1) = a {
                    if let Some(b1) = b {
                        return !std::ptr::eq(a1, b1);    
                    }
                }
                true  
            });
        }

        pub fn get_curr(&self) -> Option<i32> {
            self.cell_mutable.get()
        }

    }    
    
    #[test]
    fn set_if_works() {
        let app = App::new();

        app.set_cell(Some(33));
        assert_eq!(Some(33), app.get_curr());

        app.set_cell(Some(66));
        assert_eq!(Some(66), app.get_curr());

        app.set_cell(Some(66));
        assert_eq!(Some(66), app.get_curr());
    }

    #[test]
    fn compare_works() {
        let app1 = &App::new();
        let app2 = &App::new();

        let tst = std::ptr::eq(app1, app2);
        assert_eq!(false, tst);

        let tst = std::ptr::eq(app1, app1);
        assert_eq!(true, tst);
    }

    #[tokio::test]
    async fn map_ref_works() {
        let use_last = Mutable::new(true);
        let first = Mutable::new("Bill");
        let last = Mutable::new("Smith");
        let full_name = map_ref! {
            let use_last = use_last.signal(),
            let first = first.signal(),
            let last = last.signal() =>
            if *use_last {
                format!("{first} {last}")
            } else {
                first.to_string()
            }
        };

        let mut count = 0;
        let future = full_name.for_each(move |v| {
            match count {
                0 => {
                    println!("to compare {:?}", v);
                    assert_eq!("Bill Smith", v)
                },
                1 => {
                    println!("to compare {:?}", v);
                    assert_eq!("value Smith", v)
                },
                _ => ()
            }

            count += 1;
            async {}
        });

        let task = tokio::spawn(future);
        let mut interval = time::interval(Duration::from_millis(100));
        interval.tick().await;

        first.set("value");
        interval.tick().await;

        task.abort();
    }

    #[tokio::test]
    async fn paused_time() {
        tokio::time::pause();
        let start = std::time::Instant::now();
        tokio::time::sleep(Duration::from_millis(500)).await;
        println!("{:?}ms", start.elapsed().as_millis());
    }    

    #[tokio::test]
    async fn signal_map_test() {
        let map = MutableBTreeMap::new();

        let signal = map.signal_map();
        let future = signal.for_each(|m| {
            match m {
                Insert { key, value } => println!("insert {key}:{value}"),
                Remove { key } => println!("remove {key}"),
                Update { key, value } => println!("update {key}:{value}"),
                Replace { entries } => println!("replace {:?}", entries),
                Clear {  } => println!("clear"),
                _ => println!("other")
            }
            async {}
        });

        let task = tokio::spawn(future);
        let mut interval = time::interval(Duration::from_millis(100));
        interval.tick().await;

        let mut lock = map.lock_mut();
        // let foo = "foo";
        lock.insert("foo", 5);
        assert_eq!(lock[&"foo"], 5);

        lock.insert("bar", 10);
        assert_eq!(lock[&"bar"], 10);

        lock.remove(&"foo");
        assert!(!lock.contains_key("foo"));

        lock.insert("bar", 78);
        if let Some(v) = lock.get("bar") {
            assert_eq!(*v, 78);
        }

        let mut values = BTreeMap::new();
        values.insert("foo", 6);
        values.insert("zaz", 66);

        lock.replace(values);
        if let Some(v) = lock.get("foo") {
            assert_eq!(*v, 6);
        }

        let bar = lock.get_key_value("bar");
        assert_eq!(None, bar);

        lock.clear();

        interval.tick().await;

        task.abort();        

    }

    #[test]
    fn calculate_hash_works() {
        let a = "AAA".to_owned();
        let a_hash = calculate_hash(&a);
        println!("tst hash: {a_hash}");

        let a1 = "AAA".to_owned();
        let a1_hash = calculate_hash(&a1);
        println!("tst hash: {a1_hash}");

        assert_eq!(a_hash, a1_hash);

        let a1 = "AAB".to_owned();
        let a1_hash = calculate_hash(&a1);
        println!("tst hash: {a1_hash}");

        assert_ne!(a_hash, a1_hash);
    }


}