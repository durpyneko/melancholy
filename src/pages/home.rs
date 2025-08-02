use crate::rtml;
use crate::utils::snippets;

pub fn render() -> web_sys::Element {
    snippets::set_title("home");

    let tree = rtml! {
        div {
            div {
                class: "p-4"
                span {
                    class: "flex justify-center text-2xl"
                    "𝐖𝐄𝐋𝐂𝐎𝐌𝐄 to\u{00A0}"
                    span {
                        class: "text-[#ba3e80]" // #a63654 | #ba3e80
                            "melancholy\u{00A0}\u{00A0}"
                        }
                    span {
                        "₍₍⚞(˶˃ ꒳ ˂˶)⚟⁾⁾"
                    }
                }
                span {
                    class: "flex justify-center"
                    "malancholy is a low level personal website in rust and webassembly"
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
            div {
                class: "flex justify-center"
                div {
                    class: "flex flex-col text-2xl"
                    a {
                        href: "https://v3.durpy.dev/"
                        target: "_blank"
                        span {
                            "⤷ v3"
                        }
                    }
                    a {
                        href: "https://v2.durpy.dev/"
                        target: "_blank"
                        span {
                        "⤷ v2"
                        }
                    }
                }
            }
        }
    };

    tree
}
