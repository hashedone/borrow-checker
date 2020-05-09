struct DestroyMe;

impl Drop for DestroyMe {
    fn drop(&mut self) {
        println!("Destroyed");
    }
}

fn foo(_dm: DestroyMe) {
    println!("foo called");
}

fn main() {
    let dm = DestroyMe;
    foo(dm);
    foo(dm);
}
