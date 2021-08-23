use std::cell::{RefCell, RefMut};
use std::ops::DerefMut;
use std::sync::Arc;

fn main() {
    let my_string = Arc::new(RefCell::new(String::new()));
    let mut threads = Vec::new();
    for _ in 0..10 {
        let my_string = my_string.clone();
        let thread = std::thread::spawn(move || {
            // my_string.borrow_mut().push_str("x");
            let mut guard: RefMut<String> = my_string.borrow_mut();
            let my_string_mut: &mut String = guard.deref_mut();
            my_string_mut.push_str("x");
        });
        threads.push(thread);
    }
    for thread in threads {
        thread.join().unwrap();
    }
    dbg!(my_string);

    let my_string = &RefCell::new(String::new());
    // my_string.borrow_mut().push_str("x");
    let mut guard: RefMut<String> = my_string.borrow_mut();
    let my_string_mut: &mut String = guard.deref_mut();
    my_string_mut.push_str("x");
    dbg!(my_string);
}
