//! Abstractions for non-blocking programming.
//!
//! This crate provides a number of core abstractions for writing asynchronous code:
//! - [`Task`]:  A single eventual values produced by asynchronous computations.
//! - [`Stream`]: A series of values produced asynchronously.

#![no_std]
#![cfg_attr(feature = "generators", feature(generators, generator_trait))]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use pin_utils::pin_mut;

pub mod io;

pub mod stream;
pub use stream::Stream;

pub mod task;
pub use task::Task;

/// Indicates whether a value is available or still pending.
///
/// This differs from [core::task::Poll] because tasks aren't required to schedule themselves for wakeup.
#[must_use = "this `Poll` may be a `Pending` variant, which should be handled"]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Poll<T> {
    /// Represents that a value is immediately ready.
    Ready(T),

    /// Represents that a value is not ready yet.
    Pending,
}

impl<T> Poll<T> {
    pub fn is_ready(&self) -> bool {
        match self {
            Self::Ready(_) => true,
            Self::Pending => false,
        }
    }

    #[inline]
    pub fn map<U, F>(self, f: F) -> Poll<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Poll::Ready(t) => Poll::Ready(f(t)),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(feature = "nb")]
#[cfg_attr(docsrs, doc(cfg(feature = "nb")))]
impl<T, E> From<nb::Result<T, E>> for Poll<Result<T, E>> {
    fn from(value: nb::Result<T, E>) -> Self {
        match value {
            Ok(out) => Poll::Ready(Ok(out)),
            Err(nb::Error::Other(error)) => Poll::Ready(Err(error)),
            Err(nb::Error::WouldBlock) => Poll::Pending,
        }
    }
}

/// Extracts the successful type of a `Poll<T>`.
///
/// This macro bakes in propagation of `Pending` signals by returning early.
#[macro_export]
macro_rules! ready {
    ($e:expr $(,)?) => {
        match $e {
            $crate::Poll::Ready(t) => t,
            $crate::Poll::Pending => return $crate::Poll::Pending,
        }
    };
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
#[cfg(feature = "generators")]
#[cfg_attr(docsrs, doc(cfg(feature = "generators")))]
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
