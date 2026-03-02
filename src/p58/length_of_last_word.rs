pub fn length_of_last_word(s: &str) -> i32 {
    let mut trimmed_reverse_str_iterator = s.trim().split_ascii_whitespace();

    if let Some(item) = trimmed_reverse_str_iterator.next_back() {
        item.len() as i32
    } else {
        0
    }
}