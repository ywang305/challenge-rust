fn main() {
    squre(32);
    let x = 5;
    let y = 4;
    if x > y {
        println!("x is greater than y");
    } else {
        println!("x is less than y");
    }
    let z = if x > y { x } else { y };
    println!("{}", z);
}

fn squre(x: i32) -> i32 {
    x * x
}
