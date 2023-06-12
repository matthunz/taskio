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

    fn poll_unpin(mut self) -> Poll<Self::Output>
    where
        Self: Sized + Unpin,
    {
        Pin::new(&mut self).poll()
    }

    fn block_on(mut self) -> Self::Output
    where
        Self: Sized + Unpin,
    {
        loop {
            if let Poll::Ready(output) = (&mut self).poll_unpin() {
                break output;
            }
        }
    }
}

impl<T> Task for &mut T
where
    T: ?Sized + Task + Unpin,
{
    type Output = T::Output;

    fn poll(mut self: Pin<&mut Self>) -> Poll<Self::Output> {
        Pin::new(&mut **self).poll()
    }
}
