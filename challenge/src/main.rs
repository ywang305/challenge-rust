fn main() {
    assert_eq!("abc", trim_sppace(" abc   "));

    assert_eq!("ab", &"abcd"[0..2]);
}

fn trim_sppace(s: &str) -> &str {
    let mut start = 0;
    for (index, character) in s.chars().enumerate() {
        if character != ' ' {
            start = index;
            break;
        }
    }
    let mut end = 0;
    for (index, character) in s.chars().rev().enumerate() {
        if character != ' ' {
            end = s.len() - index;
            break;
        }
    }

    &s[start..end]
}
