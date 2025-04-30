use std::ops::{Deref, DerefMut};

pub struct BadRefCell<T> {
    value: T,
    locked: bool,
}

pub struct BadRefMut<'a, T>(&'a mut BadRefCell<T>);

impl<T> BadRefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            locked: false,
        }
    }

    pub fn lock(&self) -> BadRefMut<T> {
        assert!(!self.locked);
        // XXX: This cast is never correct. As of 1.73 it's a compiler error by default.
        let mut_self: &mut Self = unsafe {
            #[allow(invalid_reference_casting)]
            &mut *(self as *const _ as *mut _)
        };
        mut_self.locked = true;
        BadRefMut(mut_self)
    }
}

impl<'a, T> Drop for BadRefMut<'a, T> {
    fn drop(&mut self) {
        self.0.locked = false;
    }
}

impl<'a, T> Deref for BadRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0.value
    }
}

impl<'a, T> DerefMut for BadRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0.value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let x = BadRefCell::new(0);
        *x.lock() += 1;
        *x.lock() += 1;
        assert_eq!(*x.lock(), 2);
    }

    #[test]
    #[should_panic]
    fn test_exclusion() {
        let x = BadRefCell::new(42);
        let _guard1 = x.lock();
        let _guard2 = x.lock();
    }

    fn add_one(cell: &BadRefCell<i32>) {
        *cell.lock() += 1;
    }

    #[test]
    fn test_trigger_undefined_behavior() {
        let cell = BadRefCell::new(0);
        add_one(&cell);
        assert_eq!(*cell.lock(), 1);
    }
}
