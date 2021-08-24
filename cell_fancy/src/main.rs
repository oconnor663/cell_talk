use std::cell::Cell;

fn main() {
    let mut my_int = 42;
    let cell_ref: &Cell<i32> = Cell::from_mut(&mut my_int);
    let ten_cell_refs = [cell_ref; 10];
    for c in &ten_cell_refs {
        c.set(c.get() + 1);
    }
    assert_eq!(my_int, 52);

    let mut my_array = [10, 20, 30];
    let cell_ref: &Cell<[i32]> = Cell::from_mut(&mut my_array[..]);
    let slice_of_cells: &[Cell<i32>] = Cell::as_slice_of_cells(cell_ref);
    for c in slice_of_cells {
        c.set(c.get() + 1);
    }
    assert_eq!(my_array, [11, 21, 31]);
}
