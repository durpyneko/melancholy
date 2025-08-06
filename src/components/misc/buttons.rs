use crate::rtml;

struct BUTTON {
    title: Option<&'static str>,
    src: &'static str,
    href: Option<&'static str>,
}

enum ButtonTypes {
    Button,
    Stamp,
    Blinkie,
}

pub fn render() -> web_sys::Element {
    let container = rtml! {
        div {
            class: "flex justify-center"
            div {
                class: "flex flex-col mt-4 gap-1"
                /* https://adriansblinkiecollection.neocities.org/ */
                div {
                    id: "sticker-container"
                    class: "flex justify-center flex-row gap-1 items-center"
                    img {
                        class: "h-[56px] w-[99px] flex-none"
                        title: "kuru kuru"
                        src: "/images/buttons/miku_kurukuru.jpg"
                    }
                    img {
                        class: "h-[56px] w-[99px] flex-none"
                        src: "/images/buttons/d55.gif"
                    }
                    img {
                        class: "h-[56px] w-[99px] flex-none"
                        src: "/images/buttons/d18.gif"
                    }
                    img {
                        class: "h-[56px] w-[99px] flex-none"
                        src: "/images/buttons/e110.gif"
                    }
                }
                div {
                    id: "button-container"
                    class: "flex justify-center flex-row gap-1 items-center"
                    img {
                        class: "h-[31px] w-[88px] flex-none"
                        src: "/images/buttons/sharingiscaring.gif"
                    }
                }
            }
        }
    };

    /* let inner = container.first_child().unwrap(); // the inner div

    let one = button(BUTTON {
        title: Some("kuru kuru"),
        src: "/images/buttons/miku_kurukuru.jpg",
        href: None,
    });
    let two = button(BUTTON {
        title: None,
        src: "/images/buttons/sharingiscaring.gif",
        href: None,
    });

    inner.append_child(&one).unwrap();
    inner.append_child(&two).unwrap(); */
    container
}

fn button(button: BUTTON) -> web_sys::Element {
    let img = rtml! {
        img {}
    };

    if let Some(title) = button.title {
        img.set_attribute("title", title).unwrap();
    }

    img.set_attribute("src", button.src).unwrap();

    if let Some(href) = button.href {
        let a = rtml! {
            a {}
        };
        a.set_attribute("href", href).unwrap();
        a.append_child(&img).unwrap();
        a
    } else {
        img
    }
}
