fn printer<'a>(data: &'a [i32]) -> impl Fn() -> () + 'a {
    let iter = data.iter();
    move || {
        for item in iter.clone() {
            println!("Item: {}", item);
        }
    }
}

fn magic(data: &mut Vec<i32>) {
    let mut last = *data.last().unwrap();
    while last != 1 {
        if last % 2 == 0 {
            data.push(last / 2);
        } else {
            data.push(last * 3 + 1);
        }

        last = *data.last().unwrap();
    }
}

fn main() {
    let mut data = vec![871];
    magic(&mut data);
    printer(&data)();
}
