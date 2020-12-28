mod utils;


use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use std::sync::mpsc;
use web_sys::{window, console::*};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet() -> Result<(), JsValue> {
    utils::set_panic_hook();
    
    let (s1, r1) = mpsc::sync_channel::<&str>(1);

    let settimeout_callback = Closure::wrap(Box::new(move || {
        log_1(&JsValue::from_str("settimeout_callback"));
        match s1.send("done") {
            Ok(_) => log_1(&JsValue::from_str("OK")),
            Err(e) => log_2(&JsValue::from_str("Error: "),
            &JsValue::from_str(e.to_string().as_str())),
        }
        log_1(&JsValue::from_str("settimeout_callback done"));
    }) as Box<dyn Fn()>);
    window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
        settimeout_callback.as_ref().unchecked_ref(), 1000)?;
    settimeout_callback.forget();
    match r1.recv(){
        Ok(s) => log_2(&JsValue::from_str("Got: "), &JsValue::from_str(s)),
        Err(e) => log_2(&JsValue::from_str("Error: "), &JsValue::from_str(e.to_string().as_str())),
    };

    Ok(())
}
