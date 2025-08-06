use crate::rtml;
use crate::utils::snippets;

pub fn render() -> web_sys::Element {
    snippets::set_title("page not found");

    let tree = rtml! {
        div {
            class: "flex items-center justify-center h-screen"
            a {
                href: "/"
                div {
                    class: "flex justify-center"
                    img {
                        class: "w-[300px]"
                        src: "/images/d722262e277bb21e40d36ec5f2f5c773.png"
                    }
                }
                class: "flex justify-center flex-col gap-8"
                div {
                    class: "flex flex-col gap-2"
                    span {
                        class: "flex justify-center text-4xl"
                        "Page not found"
                    }
                    span {
                        class: "flex justify-center text-xl"
                        "if you know where to find it, let me know"
                    }
                    span {
                        class: "flex justify-center text-2xl bold text-[#366]"
                        "[return]"
                    }
                }
            }
        }
    };

    tree
}
