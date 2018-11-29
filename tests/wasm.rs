/* run with:

       wasm-pack test --headless --firefox
*/
extern crate wasm_bindgen_test;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate js_sys;

use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

use std::panic;

use wasm_bindgen::{JsCast};
use web_sys::*;

#[wasm_bindgen_test]
fn try_dyn_into_fails_hard() {
    /*
        this aborts with:

---- wasm::try_dyn_into_fails_hard output ----
    JS exception that was thrown:
        ReferenceError: WorkerGlobalScope is not defined
        __widl_instanceof_WorkerGlobalScope@http://127.0.0.1:54222/wasm-bindgen-test:33:12

    */
    let result = panic::catch_unwind(|| {
        if let Ok(ctx) = js_sys::global().dyn_into::<WorkerGlobalScope>() {
            assert!(ctx.origin().starts_with("http"));
        }
        else {
            assert!(false, "this will not be reached");
        }
    });
    assert!(result.is_ok(), "currently not reached");
}
