use web_sys::window;

use crate::pages;

pub struct AppRouter;

impl AppRouter {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self) {
        let route = get_current_route();
        let element = match route.as_str() {
            "/" => pages::home::render(),
            "/about" => pages::about::render(),
            _ => pages::home::render(),
        };

        let body = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap();
        body.set_inner_html(""); // clear existing content
        body.append_child(&element).unwrap();
    }
}

fn get_current_route() -> String {
    window()
        .and_then(|w| w.location().pathname().ok())
        .unwrap_or_else(|| "/".to_string())
}
