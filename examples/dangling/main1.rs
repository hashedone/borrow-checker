fn dangling(arg: i32) -> &'static i32 {
    &arg
}

fn main() {
    let answer = 42;
    let borrow = dangling(answer);
    println!("Answer: {}", borrow);
}
