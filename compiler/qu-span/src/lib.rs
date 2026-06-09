use qu_source::SourceId;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub source_id: SourceId,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct Spanned<T> {
    item: T,
    span: Span,
}

impl<T> Spanned<T> {
    pub fn new(item: T, span: Span) -> Self {
        Self { item, span }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn get(&self) -> &T {
        &self.item
    }
}

impl Span {
    pub fn new(start: usize, end: usize, source_id: SourceId) -> Self {
        Self {
            start,
            end,
            source_id,
        }
    }

    pub fn cover(self, other: Self) -> Self {
        assert!(
            self.source_id == other.source_id,
            "attempt to cover spans from different files"
        );
        Self::new(
            self.start.min(other.start),
            self.end.max(other.end),
            self.source_id,
        )
    }
}
