use crate::rtml;

pub fn render() -> web_sys::Element {
    let tree = rtml! {
        div {
            class: "absolute bottom-0 left-1/2 transform -translate-x-1/2 mb-4 text-center text-yellow-400"
            "⚠︎ melancholy is in dev ⚠︎"
        }
    };

    tree
}
