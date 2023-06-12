use crate::{Poll, Task};
use core::pin::Pin;

mod for_each;
pub use for_each::ForEach;

/// A stream of values produced asynchronously.
pub trait Stream {
    type Item;

    fn poll_next(self: Pin<&mut Self>) -> Poll<Option<Self::Item>>;

    fn for_each<T, F>(self, f: F) -> ForEach<Self, T, F>
    where
        F: FnMut(Self::Item) -> T,
        T: Task<Output = ()>,
        Self: Sized,
    {
        ForEach::new(self, f)
    }
}
