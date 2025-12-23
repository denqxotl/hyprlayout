pub fn get_layout_short_name(layout: &str) -> String {
    match layout {
        "English (US)" => "US",
        "Ukrainian" => "UA",
        _ => layout,
    }
    .to_string()
}
