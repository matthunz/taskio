#![no_std]
#[cfg_attr(feature = "generators", feature(generators, generator_trait))]
use core::{pin::Pin, task::Poll};

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
                core::task::Poll::Ready(t) => break t,
                core::task::Poll::Pending => {
                    yield;
                }
            }
        }
    };
}
