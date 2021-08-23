use std::cell::Cell;
use std::sync::Arc;

fn main() {
    let my_int = Arc::new(Cell::new(42));
    let mut threads = Vec::new();
    for _ in 0..10 {
        let my_int = my_int.clone();
        let thread = std::thread::spawn(move || {
            let my_int_copy = my_int.get();
            my_int.set(my_int_copy + 1);
        });
        threads.push(thread);
    }
    for thread in threads {
        thread.join().unwrap();
    }
    dbg!(my_int.take());

    let my_int = &Cell::new(42);
    let my_int_copy = my_int.get();
    my_int.set(my_int_copy + 1);
    dbg!(my_int.take());
}
