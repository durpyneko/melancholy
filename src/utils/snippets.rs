use phf::phf_map;
use web_sys::window;

pub static ICONS: phf::Map<&'static str, &'static str> = phf_map! {
    "dot" => "𓋰",
    "heart" => "♥",
    "star" => "★",
};

pub fn set_title(title: &str) {
    if let Some(doc) = window().and_then(|w| w.document()) {
        doc.set_title(format!("⟡ {} 𓉘 melancholy 𓉝 ⟡ ⋆｡ﾟ☁︎｡⋆｡ ﾟ☾ ﾟ｡⋆", title).as_str());
    }
}
