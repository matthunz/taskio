use crate::{ready, Poll, Stream, Task};
use core::pin::Pin;
use pin_project_lite::pin_project;

pin_project! {
    pub struct ForEach<S, T, F> {
        #[pin]
        stream: S,
        f: F,
        #[pin]
        task: Option<T>,
    }
}

impl<S, T, F> ForEach<S, T, F> {
    pub fn new(stream: S, f: F) -> Self {
        Self {
            stream,
            f,
            task: None,
        }
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
