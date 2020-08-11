use super::Scorer;
use crate::core::SegmentReader;
use crate::query::Explanation;
use crate::{DocId, Result};
use std::ops::Deref;

/// A Weight is the specialization of a Query
/// for a given set of segments.
///
/// See [`Query`](./trait.Query.html).
pub trait Weight: Send + Sync + 'static {
    /// Returns the scorer for the given segment.
    /// See [`Query`](./trait.Query.html).
    fn scorer(&self, reader: &SegmentReader) -> Result<Box<dyn Scorer>>;

    /// Returns an `Explanation` for the given document.
    fn explain(&self, reader: &SegmentReader, doc: DocId) -> Result<Explanation>;

    /// Returns the number documents within the given `SegmentReader`.
    fn count(&self, reader: &SegmentReader) -> Result<u32> {
        let mut scorer = self.scorer(reader)?;
        if let Some(delete_bitset) = reader.delete_bitset() {
            Ok(scorer.count(delete_bitset))
        } else {
            Ok(scorer.count_including_deleted())
        }
    }
}

impl Weight for Box<dyn Weight> {
    fn scorer(&self, reader: &SegmentReader) -> Result<Box<dyn Scorer>> {
        self.deref().scorer(reader)
    }
    fn explain(&self, reader: &SegmentReader, doc: DocId) -> Result<Explanation> {
        self.deref().explain(reader, doc)
    }
    fn count(&self, reader: &SegmentReader) -> Result<u32> {
        self.deref().count(reader)
    }
}
