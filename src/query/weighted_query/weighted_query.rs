use super::weighted_weight::WeightedWeight;
use crate::query::Query;
use crate::query::Weight;
use crate::Result;
use crate::Score;
use crate::Searcher;
use crate::Term;
use std::clone::Clone;
use std::collections::BTreeSet;
use std::fmt;

#[derive(Clone)]
pub struct WeightedQuery<Q: Query> {
    query: Q,
    score_weight: Score,
}

impl<Q: Query> fmt::Debug for WeightedQuery<Q> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "WeightedQuery({:?}, {:?})",
            self.query, self.score_weight
        )
    }
}

impl<Q: Query> WeightedQuery<Q> {
    /// Creates a new weighted query.
    pub fn new(query: Q, score_weight: Score) -> WeightedQuery<Q> {
        WeightedQuery {
            query: query,
            score_weight: score_weight,
        }
    }
}

impl<Q: Query + Clone> Query for WeightedQuery<Q> {
    fn weight(&self, searcher: &Searcher, scoring_enabled: bool) -> Result<Box<dyn Weight>> {
        let weight = self.query.weight(searcher, scoring_enabled)?;
        Ok(Box::new(WeightedWeight::new(weight, self.score_weight)))
    }
    fn query_terms(&self, term_set: &mut BTreeSet<Term>) {
        self.query.query_terms(term_set)
    }
}
