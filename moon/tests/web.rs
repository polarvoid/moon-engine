//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]
use std::assert;

use moon_engine::gl::get_context;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn get_now_sec() {
    use moon_engine::web::now_sec;

    assert!(now_sec() > 0.0);
}

#[wasm_bindgen_test]
fn can_get_context() {
    use moon_engine::gl::get_context;

    let canvas = get_context();
}
