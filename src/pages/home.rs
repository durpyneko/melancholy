use crate::rtml;

use crate::utils::snippets;

use crate::components::misc;

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
                    class: "flex flex-row text-2xl gap-4 underline"
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
            div {
                class: "mt-8"
            }
            div {
                class: "flex justify-center"
                div {
                    class: "triangle-down z-0 absolute"
                }
                /* div {
                    class: "triangle-up z-0 absolute"
                } */
                div {
                    class: "flex flex-col"
                    span {
                        id: "consciousness-text"
                        class: "flex justify-center text-[#92dd78] text-lg z-10"
                        "Please stand by. I am downloading your consciousness..."
                    }
                    div {
                        class: "flex justify-center z-10"
                        img {
                            class: "max-w-[300px] border-b-1 border-[#92db75]"
                            src: "/images/1588441dc87d91dcf841298321034f83.jpg"
                        }
                    }
                }
            }
        }
    };

    let buttons = misc::buttons::render();
    tree.append_child(&buttons).unwrap();

    /* let document = web_sys::window().unwrap().document().unwrap();
    let consciousness_text = document
        .get_element_by_id("consciousness-text")
        .expect("Element with ID 'consciousness-text' not found");

    let text = document.create_element("span").unwrap();
    text.set_text_content(Some("establishing uplink..."));
    consciousness_text.append_child(&text).unwrap(); */

    tree
}
