pub fn is_invalid_id(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let mid = len / 2;
    let first_half = &s[..mid];
    let second_half = &s[mid..];

    first_half == second_half
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn detects_simple_invalid_id() {
        assert!(is_invalid_id(55));
    }
}
