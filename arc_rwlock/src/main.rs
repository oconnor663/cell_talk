use std::ops::DerefMut;
use std::sync::{Arc, RwLock, RwLockWriteGuard};

fn main() {
    let my_string = Arc::new(RwLock::new(String::new()));
    let mut threads = Vec::new();
    for _ in 0..10 {
        let my_string = my_string.clone();
        let thread = std::thread::spawn(move || {
            // my_string.lock().unwrap().push_str("x");
            let mut guard: RwLockWriteGuard<String> = my_string.write().unwrap();
            let my_string_mut: &mut String = guard.deref_mut();
            my_string_mut.push_str("x");
        });
        threads.push(thread);
    }
    for thread in threads {
        thread.join().unwrap();
    }
    dbg!(my_string);
}
