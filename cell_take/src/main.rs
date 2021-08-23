use std::cell::Cell;

fn main() {
    let string_cell = &Cell::new(String::from("foo"));
    // assert_eq!(string_cell.get(), "foo");
    assert_eq!(string_cell.take(), "foo");
    assert_eq!(string_cell.take(), "");
}
