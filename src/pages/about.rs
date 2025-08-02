use crate::rtml;
use crate::utils::snippets;

pub fn render() -> web_sys::Element {
    snippets::set_title("about");

    let tree = rtml! {
        h1 {
            class: "flex justify-center"
            "about page here"
        }
    };

    tree
}
