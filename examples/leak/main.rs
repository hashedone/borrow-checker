use std::rc::Rc;
use std::cell::RefCell;

struct A {
    data: i32,
    ptr: RefCell<Option<Rc<B>>>,
}

struct B {
    data: i32,
    ptr: Rc<A>,
}

fn main() {
    let a = Rc::new(A {
        data: 42,
        ptr: RefCell::new(None)
    });
    // `b::data` points to `a`
    let b = Rc::new(B {
        data: 1024,
        ptr: a.clone()
    });
    // `a::data` points to `b`
    a.ptr.replace(Some(b.clone()));

    println!(
        "A data: {}, B data: {}",
        b.ptr.data,
        a.ptr.borrow().as_ref().unwrap().data
    )
}

