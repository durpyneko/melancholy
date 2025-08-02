use crate::rtml;
use crate::utils::snippets;

pub fn render() -> web_sys::Element {
    snippets::set_title("page not found");

    let tree = rtml! {
        div {
            class: "flex items-center justify-center h-screen"
            a {
                href: "/"
                img {
                    class: "w-[300px]"
                    src: "/images/d722262e277bb21e40d36ec5f2f5c773.png"
                }
                class: "flex flex-col gap-8"
                div {
                    class: "flex flex-col gap-2"
                    span {
                        class: "text-3xl flex justify-center"
                        "Page not found"
                    }
                    span {
                        class: "text-3xl flex justify-center bold text-[#366]"
                        "[return]"
                    }
                }
            }
        }
    };

    tree
}
