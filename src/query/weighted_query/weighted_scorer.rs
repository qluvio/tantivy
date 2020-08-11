use crate::docset::{DocSet, SkipResult};
use crate::query::{Explanation, Scorer};
use crate::DocId;
use crate::Score;

pub struct WeightedScorer<S: DocSet + Scorer> {
    scorer: S,
    score_weight: Score,
}

impl<S: DocSet + Scorer> WeightedScorer<S> {
    pub fn new(scorer: S, score_weight: Score) -> WeightedScorer<S> {
        WeightedScorer {
            scorer: scorer,
            score_weight: score_weight,
        }
    }
}


impl<S: DocSet + Scorer> DocSet for WeightedScorer<S> {
    fn advance(&mut self) -> bool {
        self.scorer.advance()
    }

    fn skip_next(&mut self, target: DocId) -> SkipResult {
        self.scorer.skip_next(target)
    }

    fn doc(&self) -> DocId {
        self.scorer.doc()
    }

    fn size_hint(&self) -> u32 {
        self.scorer.size_hint()
    }
}

impl<S: DocSet + Scorer> Scorer for WeightedScorer<S> {
    fn score(&mut self) -> Score {
        self.scorer.score() + self.score_weight
    }
}
