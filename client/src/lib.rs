#![allow(dead_code, unused_variables)]

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wee_alloc::WeeAlloc;
use stdweb::web;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

dirmod::all!();

#[wasm_bindgen]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("main-canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

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
        .unwrap();
    canvas.set_fill_style(&JsValue::from_str("black"));
    canvas.fill_rect(0.0, 0.0, width as f64, height as f64);

    if let Err(err) = choose_server("ws://sofe.pmmp.io:15678") {
        web::alert(&format!("Error: {}", err));
    }
}
