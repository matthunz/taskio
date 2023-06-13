use pin_project_lite::pin_project;

use crate::{Poll, Task};
use core::pin::Pin;

pub trait Read {
    type Error;

    fn poll_read(self: Pin<&mut Self>, buf: &mut [u8]) -> Poll<Result<usize, Self::Error>>;

    fn poll_read_unpin(mut self, buf: &mut [u8]) -> Poll<Result<usize, Self::Error>>
    where
        Self: Sized + Unpin,
    {
        Pin::new(&mut self).poll_read(buf)
    }

    fn read(self, buf: &mut [u8]) -> ReadTask<'_, Self>
    where
        Self: Sized,
    {
        ReadTask { buf, read: self }
    }
}

impl<T> Read for &mut T
where
    T: ?Sized + Read + Unpin,
{
    type Error = T::Error;

    fn poll_read(mut self: Pin<&mut Self>, buf: &mut [u8]) -> Poll<Result<usize, Self::Error>> {
        Pin::new(&mut **self).poll_read(buf)
    }
}

pin_project! {
    pub struct ReadTask<'a, R> {
        buf: &'a mut [u8],
        #[pin]
        read: R
    }
}

impl<R> Task for ReadTask<'_, R>
where
    R: Read,
{
    type Output = Result<usize, R::Error>;

    fn poll(self: Pin<&mut Self>) -> Poll<Self::Output> {
        let me = self.project();
        me.read.poll_read(me.buf)
    }
}
