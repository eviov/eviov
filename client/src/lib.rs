#![allow(dead_code, unused_variables)]
#![warn(
    unused_results,
    unused_qualifications,
    variant_size_differences,
    clippy::checked_conversions,
    clippy::needless_borrow,
    clippy::shadow_unrelated,
    clippy::wrong_pub_self_convention
)]
#![deny(
    anonymous_parameters,
    bare_trait_objects,
    clippy::as_conversions,
    clippy::clone_on_ref_ptr,
    clippy::float_cmp_const,
    clippy::if_not_else,
    clippy::indexing_slicing,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]
#![cfg_attr(not(debug_assertions), deny(warnings, clippy::dbg_macro,))]

use stdweb::web;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasm_bindgen::JsCast;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

mod conn;
mod pool;

#[wasm_bindgen]
pub fn entry() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    let canvas = web_sys::window()
        .expect("DOM error")
        .document()
        .expect("DOM error")
        .get_element_by_id("main-canvas")
        .expect("Canvas element not found")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Canvas element is not a <canvas>");

    let (width, height) = (canvas.width(), canvas.height());

    let canvas = match canvas.get_context("2d").ok().flatten() {
        Some(canvas) => canvas,
        None => {
            web::alert("2D canvas is not available");
            return;
        }
    };

    let canvas = canvas
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("2D canvas rendering is not supported");
    canvas.set_fill_style(&JsValue::from_str("black"));
    canvas.fill_rect(0.0, 0.0, f64::from(width), f64::from(height));
}
