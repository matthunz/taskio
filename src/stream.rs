use crate::Poll;
use core::pin::Pin;

pub trait Stream {
    type Item;

    fn poll_next(self: Pin<&mut Self>) -> Poll<Option<Self::Item>>;
}
