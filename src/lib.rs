use wasm_bindgen::prelude::*;

mod app;
mod components;
mod pages;
mod utils;

#[wasm_bindgen(start)]
pub fn start() {
    app::AppRouter::new().render();
}
