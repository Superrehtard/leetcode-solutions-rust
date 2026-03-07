pub fn longest_palindrome(s: String) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut l: usize = 0;
    let mut r: usize = 0;
    let mut best = (0, 0);
    let byte_array = s.as_bytes();

    for i in 1..s.len() {
        (l, r) = expand(byte_array, i, i);

        if r-l > best.1 - best.0 {
            best = (l, r); 
        }

        (l, r) = expand(byte_array, i-1, i);

        if r.saturating_sub(l) > best.1 - best.0 {
            best = (l, r);
        }
    }

    s[best.0..=best.1].to_string()
}

pub fn expand(s: &[u8], mut l: usize, mut r: usize) -> (usize, usize) {
    while l > 0 && r < s.len()-1 {
        if s[l] == s[r] {
            l -= 1;
            r += 1;
        } else {
            break;
        }
    }

    if s[l] == s[r] {
        (l, r)
    } else {
        (l+1, r-1)
    }
}