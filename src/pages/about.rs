use crate::rtml;

pub fn render() -> web_sys::Element {
    let tree = rtml! {
        h1 {
            "about page"
        }
    };

    tree
}
