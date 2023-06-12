use crate::Poll;
use core::pin::Pin;

#[cfg(feature = "generators")]
mod from_generator;
#[cfg(feature = "generators")]
pub use from_generator::{from_generator, FromGenerator};

mod ready;
pub use ready::{ready, Ready};

/// An asynchronous computation.
///
/// This is similar to [`core::future::Future`] but without context.
pub trait Task {
    type Output;

    fn poll(self: Pin<&mut Self>) -> Poll<Self::Output>;
}
