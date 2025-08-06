use crate::rtml;

pub fn render() -> web_sys::Element {
    let tree = rtml! {
        div {
            class: "flex justify-center mb-4 text-center text-yellow-400"
            "｡° ⚠︎ melancholy is in dev ⚠︎ °｡"
        }
    };

    tree
}
