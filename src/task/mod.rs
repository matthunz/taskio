use crate::Poll;
use core::pin::Pin;

#[cfg(feature = "generators")]
mod from_generator;
#[cfg(feature = "generators")]
pub use from_generator::{from_generator, FromGenerator};

mod ready;
pub use ready::{ready, Ready};

/// An asynchronous computation.
pub trait Task {
    type Output;

    fn poll(self: Pin<&mut Self>) -> Poll<Self::Output>;
}

/// ```
/// #![feature(generators)]
///
/// use taskio::{pin_mut, task, wait, Task};
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
