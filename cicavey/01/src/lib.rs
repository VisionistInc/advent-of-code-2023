pub mod work;

use wasm_bindgen::prelude::*;

//use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn driver(input: String) {
    // let console_log =  |s: String| console::log_1(&s.into());
    let log = |s: String| {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        // Manufacture the element we're gonna append
        let val = document.create_element("div").unwrap();
        val.set_text_content(Some(&s));

        body.append_child(&val).expect("Failed to append");
    };

    work::run(&input, log);
}