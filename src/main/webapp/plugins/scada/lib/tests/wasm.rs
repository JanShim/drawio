use wasm_bindgen_test::*;
use mockall::{self, mock};




wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1, 1);
}

// #[wasm_bindgen_test]
// fn fail() {
//     assert_eq!(1, 2);
// }

// #[wasm_bindgen_test]
// fn cell_works() {
//     mock! {
//         MxCell {}
//         impl Clone for MxCell {
//             fn get_diagram_meta(&self) -> scada_diagram::meta::Meta  // Result<scada_diagram::meta::Meta, JsValue>;
//         }
//     }
//     assert_eq!(1, 2);
// }




