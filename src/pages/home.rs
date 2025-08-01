use crate::rtml;
use crate::utils::snippets;

pub fn render() -> web_sys::Element {
    snippets::set_title("Home / Melancholy");

    let tree = rtml! {
        div {
            style: "padding: 40px;"
            span {
                "hello from nyancholy"
                style: "font-size: 40px;"
            }
            br {}
            img {
                src: "/images/db21a47f1e5103abeb462d74c365fbe8.jpg"
                style: "height: 400px; margin: 10px"
            }
            br {}
            span {
                "Hello World!"
                style: "font-size: 20px;"
            }
        }
    };

    tree
}
