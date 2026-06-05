use chumsky::span::{SimpleSpan, Span as _};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Ctx(pub usize);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Span(SimpleSpan<usize, Ctx>);

impl Default for Span {
    fn default() -> Self {
        Self(SimpleSpan::<usize, Ctx>::new(Ctx(0), 0..0))
    }
}

impl Span {
    #[must_use]
    pub fn new(context: Ctx, range: core::ops::Range<usize>) -> Self {
        Self(SimpleSpan::<usize, Ctx>::new(context, range))
    }

    #[must_use]
    pub const fn inner(&self) -> SimpleSpan<usize, Ctx> {
        self.0
    }
}

impl chumsky::span::Span for Span {
    type Context = Ctx;

    type Offset = usize;

    fn new(context: Self::Context, range: core::ops::Range<Self::Offset>) -> Self {
        Self::new(context, range)
    }

    fn context(&self) -> Self::Context {
        self.0.context()
    }

    fn start(&self) -> Self::Offset {
        self.0.start()
    }

    fn end(&self) -> Self::Offset {
        self.0.end()
    }
}
