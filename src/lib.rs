#![allow(unused_imports)]
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

#[wasm_bindgen]
pub fn greet() -> Result<String, JsValue> {
    let origin = self_()?.origin();
    Ok(format!("Hello from {}", origin))
}

trait WindowOrWorkerGlobalScope {
    fn origin(&self) -> String;
}

impl WindowOrWorkerGlobalScope for Window {
    fn origin(&self) -> String {
        self.origin()
    }
}

impl WindowOrWorkerGlobalScope for WorkerGlobalScope {
    fn origin(&self) -> String {
        self.origin()
    }
}

//
// Does not compile:
//
// fn self_() -> Result<impl WindowOrWorkerGlobalScope, JsValue> {
//     let global = js_sys::global();
//     if js_sys::eval("typeof WorkerGlobalScope !== 'undefined'")?.as_bool().unwrap() {
//         Ok(global.dyn_into::<WorkerGlobalScope>()?)
//     }
//     else {
//         Ok(global.dyn_into::<Window>()?)
//     }
// }


fn self_() -> Result<Box<dyn WindowOrWorkerGlobalScope>, JsValue> {
    let global = js_sys::global();
    if js_sys::eval("typeof WorkerGlobalScope !== 'undefined'")?.as_bool().unwrap() {
        Ok(Box::new(global.dyn_into::<WorkerGlobalScope>()?))
    }
    else {
        Ok(Box::new(global.dyn_into::<Window>()?))
    }
}
