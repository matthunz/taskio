use pin_project_lite::pin_project;
use crate::{ready, Poll, Task};
use core::pin::Pin;

pub trait Stream {
    type Item;

    fn poll_next(self: Pin<&mut Self>) -> Poll<Option<Self::Item>>;

    fn for_each<T, F>(self, f: F) -> ForEach<Self, T, F>
    where
        F: FnMut(Self::Item) -> T,
        T: Task<Output = ()>,
        Self: Sized,
    {
        ForEach {
            stream: self,
            f,
            task: None,
        }
    }
}

pin_project! {
    pub struct ForEach<S, T, F> {
        #[pin]
        stream: S,
        f: F,
        #[pin]
        task: Option<T>,
    }
}

impl<S, T, F> Task for ForEach<S, T, F>
where
    S: Stream,
    F: FnMut(S::Item) -> T,
    T: Task<Output = ()>,
{
    type Output = ();

    fn poll(self: Pin<&mut Self>) -> Poll<Self::Output> {
        let mut me = self.project();
        loop {
            if let Some(task) = me.task.as_mut().as_pin_mut() {
                ready!(task.poll());
                me.task.set(None);
            } else if let Some(item) = ready!(me.stream.as_mut().poll_next()) {
                me.task.set(Some((me.f)(item)));
            } else {
                break;
            }
        }
        Poll::Ready(())
    }
}
