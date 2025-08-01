use web_sys::window;

pub fn set_title(title: &str) {
    if let Some(doc) = window().and_then(|w| w.document()) {
        doc.set_title(title);
    }
}
