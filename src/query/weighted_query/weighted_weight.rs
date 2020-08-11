use super::weighted_scorer::WeightedScorer;
use crate::core::SegmentReader;
use crate::query::Weight;
use crate::query::{Explanation, Scorer};
use crate::DocId;
use crate::Result;
use crate::Score;

pub struct WeightedWeight<W: Weight> {
    pub weight: W,
    pub score_weight: Score,
}

impl<W: Weight> Weight for WeightedWeight<W> {
    fn scorer(&self, reader: &SegmentReader) -> Result<Box<dyn Scorer>> {
        let scorer = self.weight.scorer(reader)?;
        Ok(Box::new(WeightedScorer::new(scorer, self.score_weight)))
    }

    fn explain(&self, reader: &SegmentReader, doc: DocId) -> Result<Explanation> {
        self.weight.explain(reader, doc)
    }

    fn count(&self, reader: &SegmentReader) -> Result<u32> {
        self.weight.count(reader)
    }
}

impl<W: Weight> WeightedWeight<W> {
    pub fn new(weight: W, score_weight: Score) -> WeightedWeight<W> {
        WeightedWeight {
            weight: weight,
            score_weight: score_weight,
        }
    }
}
