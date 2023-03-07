#![doc = include_str!("../README.md")]

use leptos_reactive::SignalUpdate;
use std::{iter, ops};

/// Toggles this signal between two values
pub trait Toggle<I> {
    fn tracked_toggle(&self);
}

impl<T, I> Toggle<I> for T
where
    T: SignalUpdate<I>,
    I: ops::Not<Output = I> + Clone,
{
    #[inline]
    fn tracked_toggle(&self) {
        self.update(|v| *v = !v.clone())
    }
}

/// Adds a value to this signal.
pub trait AddAssign<I, Rhs> {
    fn tracked_add(&self, rhs: Rhs);
}

impl<T, I, Rhs> AddAssign<I, Rhs> for T
where
    T: SignalUpdate<I>,
    I: ops::AddAssign<Rhs>,
{
    #[inline]
    fn tracked_add(&self, rhs: Rhs) {
        self.update(|v| *v += rhs)
    }
}

/// Subtracts a value from this signal.
pub trait SubAssign<I, Rhs> {
    fn tracked_sub(&self, rhs: Rhs);
}

impl<T, I, Rhs> SubAssign<I, Rhs> for T
where
    T: SignalUpdate<I>,
    I: ops::SubAssign<Rhs>,
{
    #[inline]
    fn tracked_sub(&self, rhs: Rhs) {
        self.update(|v| *v -= rhs)
    }
}

/// Multiplies a value with this signal.
pub trait MulAssign<I, Rhs> {
    fn tracked_mul(&self, rhs: Rhs);
}

impl<T, I, Rhs> MulAssign<I, Rhs> for T
where
    T: SignalUpdate<I>,
    I: ops::MulAssign<Rhs>,
{
    #[inline]
    fn tracked_mul(&self, rhs: Rhs) {
        self.update(|v| *v *= rhs)
    }
}

/// Divides a value with this signal
pub trait DivAssign<I, Rhs> {
    fn tracked_div(&self, rhs: Rhs);
}

impl<T, I, Rhs> DivAssign<I, Rhs> for T
where
    T: SignalUpdate<I>,
    I: ops::DivAssign<Rhs>,
{
    #[inline]
    fn tracked_div(&self, rhs: Rhs) {
        self.update(|v| *v /= rhs)
    }
}

pub trait Extend<I, A> {
    fn tracked_extend<C>(&self, iter: C)
    where
        C: IntoIterator<Item = A>;
}

impl<T, I, A> Extend<I, A> for T
where
    T: SignalUpdate<I>,
    I: iter::Extend<A>,
{
    fn tracked_extend<C>(&self, iter: C)
    where
        C: IntoIterator<Item = A>,
    {
        self.update(|v| iter::Extend::extend(v, iter))
    }
}

pub trait TrackedVec<V, T> {
    fn tracked_push(&self, value: T);
    fn tracked_pop(&self) -> Option<Option<T>>;
    fn tracked_append(&self, other: &mut Vec<T>);
    fn tracked_clear(&self);
    fn tracked_insert(&self, index: usize, element: T);
    fn tracked_remove(&self, index: usize) -> Option<T>;
}

impl<V, T> TrackedVec<V, T> for V
where
    V: SignalUpdate<Vec<T>>,
{
    fn tracked_push(&self, value: T) {
        self.update(|v| v.push(value))
    }

    fn tracked_pop(&self) -> Option<Option<T>> {
        self.try_update(|v| v.pop())
    }

    fn tracked_append(&self, other: &mut Vec<T>) {
        self.update(|v| v.append(other))
    }

    fn tracked_clear(&self) {
        self.update(|v| v.clear())
    }

    fn tracked_insert(&self, index: usize, element: T) {
        self.update(|v| v.insert(index, element))
    }

    fn tracked_remove(&self, index: usize) -> Option<T> {
        self.try_update(|v| v.remove(index))
    }
}

#[cfg(test)]
mod tests {
    use leptos_reactive::{create_runtime, create_scope, create_signal, Scope, SignalGet};

    fn with_scope(func: impl Fn(Scope) + 'static) {
        create_scope(create_runtime(), func).dispose()
    }

    #[test]
    fn should_toggle() {
        with_scope(|cx| {
            use super::Toggle;

            let (read, write) = create_signal(cx, false);

            write.tracked_toggle();

            assert!(read.get())
        })
    }

    #[test]
    fn should_add() {
        with_scope(|cx| {
            use super::AddAssign;

            let (read_i32, write_i32) = create_signal(cx, 57);
            let (read_str, write_str) = create_signal(cx, String::new());

            write_i32.tracked_add(12);
            write_str.tracked_add("Hello ");
            write_str.tracked_add("World!");

            assert_eq!(read_i32.get(), 69);
            assert_eq!(read_str.get(), "Hello World!");
        })
    }

    #[test]
    fn should_extend() {
        with_scope(|cx| {
            use super::Extend;

            let (read_str, write_str) = create_signal(cx, String::new());

            write_str.tracked_extend(vec!["Hello", " World!"]);

            assert_eq!(read_str.get(), "Hello World!");
        })
    }

    #[test]
    fn should_track_vec() {
        with_scope(|cx| {
            use super::TrackedVec;

            let (read_vec, write_vec) = create_signal(cx, vec![1, 2, 3]);

            assert_eq!(write_vec.tracked_pop(), Some(Some(3)));
            assert_eq!(read_vec.get(), vec![1, 2]);

            write_vec.tracked_push(5);

            assert_eq!(read_vec.get(), vec![1, 2, 5]);

            write_vec.tracked_clear();

            assert_eq!(read_vec.get(), Vec::new() as Vec<i32>);
        })
    }

}
