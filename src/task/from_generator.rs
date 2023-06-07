use crate::{Poll, Task};
use core::{
    marker::PhantomData,
    ops::{Generator, GeneratorState},
    pin::Pin,
};
use pin_project_lite::pin_project;

pub fn from_generator<O, G: Generator<(), Yield = (), Return = O>>(
    generator: G,
) -> FromGenerator<O, G> {
    FromGenerator {
        generator,
        _output: PhantomData,
    }
}

pin_project! {
    pub struct FromGenerator<O, G> {
        #[pin]
        generator: G,
        _output: PhantomData<O>,
    }
}

impl<O, G> Task for FromGenerator<O, G>
where
    G: Generator<(), Yield = (), Return = O>,
{
    type Output = O;

    fn poll(self: Pin<&mut Self>) -> Poll<Self::Output> {
        match self.project().generator.resume(()) {
            GeneratorState::Yielded(()) => Poll::Pending,
            GeneratorState::Complete(output) => Poll::Ready(output),
        }
    }
}
