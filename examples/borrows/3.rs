fn main() {
    let mut number = 41;
    let borrow = &number;
    let mut_borrow = &mut number;
    *mut_borrow += 1;
    println!("Borrow: {}", borrow);
}

