fn borrow<'a>(arg: &'a i32) -> &'a i32 {
    &arg
}

fn main() {
    let answer = 42;
    let borrow = borrow(&answer);
    println!("Answer: {}", borrow);
}
