use std::cell::Cell;
use std::sync::Arc;

fn main() {
    let my_string = Arc::new(Cell::new(String::new()));
    let mut threads = Vec::new();
    for _ in 0..10 {
        let my_string = my_string.clone();
        let thread = std::thread::spawn(move || {
            let mut taken_string: String = my_string.take();
            taken_string.push_str("x");
            my_string.set(taken_string);
        });
        threads.push(thread);
    }
    for thread in threads {
        thread.join().unwrap();
    }
    dbg!(my_string.take());
}
