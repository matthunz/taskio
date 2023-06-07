#![no_std]
#[cfg_attr(feature = "generators", feature(generators, generator_trait))]
use core::pin::Pin;

/// Indicates whether a value is available or still pending.
/// This differs from [core::task::Poll] because tasks don't schedule themselves for wakeup.
#[must_use = "this `Poll` may be a `Pending` variant, which should be handled"]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Poll<T> {
    /// Represents that a value is immediately ready.
    Ready(T),

    /// Represents that a value is not ready yet.
    Pending,
}

pub trait Task {
    type Output;

    fn poll(self: Pin<&mut Self>) -> Poll<Self::Output>;
}

/// ```
/// #![feature(generators)]
///
/// use halio::{task, wait, Task};
/// use futures::pin_mut;
///
/// let ready = task::ready(());
/// pin_mut!(ready);
///
/// let task = task::from_generator(|| {
///     wait!(ready.as_mut());
/// });
/// pin_mut!(task);
///
/// assert!(task.poll().is_ready());
/// ```
#[macro_export]
macro_rules! wait {
    ($e:expr $(,)?) => {
        loop {
            match $e.poll() {
                $crate::Poll::Ready(t) => break t,
                $crate::Poll::Pending => {
                    yield;
                }
            }
        }
    };
}
