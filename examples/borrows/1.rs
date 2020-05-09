fn main() {
    let mut number: i32 = 41;
    let borrow: &i32 = &number;
    println!("Number: {}, borrow: {}", number, borrow);
    let mut_borrow: &mut i32 = &mut number;
    *mut_borrow += 1;
    println!("Answer: {}", number);
}
