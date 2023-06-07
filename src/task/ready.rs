use super::Task;
use crate::Poll;
use core::pin::Pin;

pub fn ready<T>(value: T) -> Ready<T> {
    Ready { value: Some(value) }
}

pub struct Ready<T> {
    value: Option<T>,
}

impl<T: Unpin> Task for Ready<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>) -> Poll<Self::Output> {
        let value = self
            .value
            .take()
            .expect("Ready task polled after completion.");
        Poll::Ready(value)
    }
}
