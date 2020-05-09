fn borrow<'a>(arg: &'a i32) -> &'a i32 {
    &arg
}

fn main() {
    let borrow = {
        let answer = 42;
        borrow(&answer)
    };
    println!("Answer: {}", borrow);
}
