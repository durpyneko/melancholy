#[macro_export]
macro_rules! rtml {
    ($($tokens:tt)*) => {{
        use web_sys::Element;

        fn create_dom() -> Element {
            let document = web_sys::window().unwrap().document().unwrap();
            $crate::rtml_internal!(@nodes document, $($tokens)*)
        }

        create_dom()
    }};
}

#[macro_export]
macro_rules! rtml_internal {
    (@nodes $doc:ident, $tag:ident { $($inside:tt)* }) => {{
        let el = $doc.create_element(stringify!($tag)).unwrap();
        $crate::rtml_internal!(@content $doc, el, $($inside)*);
        el
    }};

    (@content $doc:ident, $el:ident, ) => {}; // End

    (@content $doc:ident, $el:ident, $key:ident : $val:literal $($rest:tt)*) => {{
        $el.set_attribute(stringify!($key), $val).unwrap();
        $crate::rtml_internal!(@content $doc, $el, $($rest)*);
    }};

    (@content $doc:ident, $el:ident, $tag:ident { $($inner:tt)* } $($rest:tt)*) => {{
        let child = $crate::rtml_internal!(@nodes $doc, $tag { $($inner)* });
        $el.append_child(&child).unwrap();
        $crate::rtml_internal!(@content $doc, $el, $($rest)*);
    }};

    (@content $doc:ident, $el:ident, $text:literal $($rest:tt)*) => {{
        let text_node = $doc.create_text_node($text);
        $el.append_child(&text_node).unwrap();
        $crate::rtml_internal!(@content $doc, $el, $($rest)*);
    }};
}
