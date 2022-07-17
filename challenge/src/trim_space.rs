
fn trim_sppace(s: &str) -> &str {
    let mut start = 0;
    for (index, character) in s.chars().enumerate() {
      if(character != ' ') {
        start = index;
        break;
      }
    }
    let mute end = 0;
    for(index , character) in s.chars().rev().enumerate() {
      if character != ' ' {
        end = s.len() - index;
        break;
      }
    }
    &s[start..end]
}

#[test]
fn test1() {
  assert_eq!("abc", trim_sppace(" abc   "))
}