fn main() {
    // 等价
    println!("{}", (2.0_f64).sqrt());
    println!("{}", f64::sqrt(2.0));
}

/**
 * Unlike C and C++, Rust performs almost no numeric conversions implicitly.
 * If a function expects an f64 argument, it’s an error to pass an i32 value as the argument.
 */
#[test]
fn test_float() {
    // INFINITY, MIN, MAX
    assert!((-1. / f32::INFINITY).is_sign_negative());
    assert_eq!(-f32::MIN, f32::MAX);

    // float can exactly ==
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.); // exactly 5.0, per IEEE
    assert_eq!((-1.01f64).floor(), -2.0);

    // dividable
    assert_eq!(1.0 / 0.0, f32::INFINITY);
    assert!(1.0 == 1.);
}

/**
 * Rust, is very strict:
 * Rust’s as operator can convert bool values to integer types,
 * However, as won’t convert in the other direction, from numeric types to bool.
 * Instead, you must write out an explicit comparison like x != 0
 */
#[test]
fn test_bool() {
    // Rust’s as operator can convert bool values to integer types:
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);
}

#[test]
fn test_char() {
    // 字符 -> ascii
    assert_eq!('*' as i32, 42);
    assert_eq!('ಠ' as u16, 0xca0);
    assert_eq!('ಠ' as i8, -0x60);
    // ascii -> 字符
    assert_eq!(char::from_u32(97), Some('a'));

    // some useful methods
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!('ಠ'.len_utf8(), 3);
    assert_eq!(std::char::from_digit(2, 10), Some('2'));
}

/**
 * tuples that contain a single value.
 * The literal ("lonely hearts",) is a tuple containing a single string; its type is (&str,).
 */
#[test]
fn tuples() {
    // fn split_at(&self, mid: usize) -> (&str, &str);
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    //This is more legible than the equivalent:
    let text = "I see the eigenvalue in thine eye";
    let temp = text.split_at(21);
    let head = temp.0;
    let tail = temp.1;
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
}
