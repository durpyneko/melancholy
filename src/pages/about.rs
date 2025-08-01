use crate::rtml;
use crate::utils::snippets;

pub fn render() -> web_sys::Element {
    snippets::set_title("About / Melancholy");

    let tree = rtml! {
        h1 {
            "about page here"
        }
    };

    tree
}
