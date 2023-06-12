#![no_std]
#![cfg_attr(feature = "generators", feature(generators, generator_trait))]

pub mod stream;

pub mod task;
pub use task::Task;

pub use futures_util::pin_mut;

/// Indicates whether a value is available or still pending.
///
/// This differs from [core::task::Poll] because tasks don't schedule themselves for wakeup.
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
