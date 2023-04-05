pub fn strip_last_char(s: &str) -> String {
    match s.char_indices().rev().next() {
        Some((last_char_index, _)) => s[..last_char_index].to_string(),
        None => String::new(),
    }
}
