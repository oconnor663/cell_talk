use std::cell::Cell;

/// XXX: This function is unsound.
pub fn clone_cell<T: Clone>(cell: &Cell<T>) -> T {
    let inner_ref: &T = unsafe { &*cell.as_ptr() };
    inner_ref.clone()
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;

    use super::*;

    #[test]
    fn try_it() {
        let cell = Cell::new(String::from("hello world"));
        let s: String = clone_cell(&cell);
        assert_eq!(s, "hello world");
    }

    #[test]
    fn break_it() {
        #[derive(Default)]
        struct Weird {
            value: i32,
            other: Rc<Cell<Option<Weird>>>,
        }

        impl Clone for Weird {
            fn clone(&self) -> Self {
                let value_ref: &i32 = &self.value;
                let first_read = *value_ref;

                self.other.set(Some(Weird::default()));

                let second_read = *value_ref;
                assert_eq!(first_read, second_read, "&i32 can't be modified");

                Weird::default()
            }
        }

        let weird = Rc::new(Cell::new(None));
        weird.set(Some(Weird {
            value: 42,
            other: Rc::clone(&weird),
        }));

        clone_cell(&weird);
    }
}
