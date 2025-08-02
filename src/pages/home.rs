use crate::rtml;
use crate::utils::snippets;

pub fn render() -> web_sys::Element {
    snippets::set_title("home");

    let tree = rtml! {
        div {
            span {
                class: "flex justify-center text-2xl p-4"
                "𝐖𝐄𝐋𝐂𝐎𝐌𝐄 to\u{00A0}"
                span {
                    class: "text-[#ba3e80]" // #a63654 | #ba3e80
                    "melancholy\u{00A0}\u{00A0}"
                }
                span {
                    "₍₍⚞(˶˃ ꒳ ˂˶)⚟⁾⁾"
                }
            }
            div {
                class: "flex justify-center"
                div {
                    class: "flex-col"
                    img {
                        class: "w-[200px]"
                        src: "/images/db21a47f1e5103abeb462d74c365fbe8.jpg"
                    }
                    span {
                        class: "flex justify-center"
                        "꒷꒦꒷꒦꒷꒦꒷꒦꒷꒦꒷꒦꒷꒦꒷꒦꒷"
                    }
                }
            }
        }
    };

    tree
}
