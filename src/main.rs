#![recursion_limit = "1024"]

mod app;
mod components;
mod example;
mod index;
mod layouts;

use wasm_bindgen::prelude::*;

pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::Model>();
    Ok(())
}
