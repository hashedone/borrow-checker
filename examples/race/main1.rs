fn next_collatz(collatz: &std::sync::Mutex<Vec<i32>>) {
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
    let data = std::sync::Mutex::new(vec![12]);
    let mut poll = vec![];

    for _ in 0..20 {
        let t = std::thread::spawn(|| next_collatz(&data));
        poll.push(t);
    }

    for t in poll {
        t.join().unwrap();
    }

    for next in data.lock().unwrap().iter() {
        println!("Next: {}", next);
    }
}
