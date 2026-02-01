/**
 * Refer to more notes in Notion
 */
fn is_palindrome(s: String) -> bool {
    let clean_string: Vec<char> = s.chars()
                            .filter(|c| c.is_ascii_alphanumeric())
                            .map(|c| c.to_ascii_lowercase())
                            .collect::<Vec<char>>();

    // len() returns a usize so without saturating_sub(1), I would need another check before this
    // for empty clean_string character vector.
    // Two pointers
    let (mut left, mut right) = (0, clean_string.len().saturating_sub(1));
    while left < right {
        if clean_string[left] == clean_string[right] {
            left += 1;
            right -= 1;
        } else {
            return false;
        }
    }

    true
}