use std::sync::Mutex;

fn next_collatz(collatz: &Mutex<Vec<i32>>) {
    let mut collatz = collatz.lock().unwrap();
    let prev = *collatz.last().unwrap();
    if prev != 1 {
        if prev % 2 == 0 {
            collatz.push(prev / 2);
        } else {
            collatz.push(prev * 3 + 1);
        }
    }
}

fn main() {
    let data = Mutex::new(vec![12]);

    crossbeam::scope(|s| {
        for _ in 0..20 {
            s.spawn(|_| next_collatz(&data));
        }
    })
    .unwrap();

    for next in data.into_inner().unwrap() {
        println!("Next: {}", next);
    }
}
