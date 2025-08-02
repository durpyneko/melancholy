use crate::rtml;

pub fn render() -> web_sys::Element {
    let tree = rtml! {
        div {
            class: "flex items-center justify-center h-screen"
            a {
                href: "/"
                class: "flex flex-col gap-8"
                span {
                    class: "text-3xl flex justify-center"
                    "Page not found"
                }
                img {
                    class: "w-[300px]"
                    src: "/images/d722262e277bb21e40d36ec5f2f5c773.png"
                }
            }
        }
    };

    tree
}
