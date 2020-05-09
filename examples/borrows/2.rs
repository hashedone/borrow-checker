fn main() {
    let number = 41;
    let answer = 42;
    let mut borrow = &number;
    println!("Borrow: {}", borrow);
    borrow = &answer;
    println!("Borrow: {}", borrow);
}

