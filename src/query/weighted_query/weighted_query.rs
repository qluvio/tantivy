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

/// A Weighted query is a query that wraps an underlying query
/// with a Score for that query
///
/// The Score associated contains two elements :
/// * `rank`       - positive interger to affect the final scoring order (high priority)
/// * `score`      - search engine score coming from the underlying query (low priority)
///
/// The Score returned by a WeightedQuery is equal to the Score returned by
/// the underlying query, plus the constant WeightedQuery Score
///
/// ```rust
/// use tantivy::collector::{Count, TopDocs};
/// use tantivy::query::{TermQuery, WeightedQuery};
/// use tantivy::schema::{Schema, TEXT, IndexRecordOption};
/// use tantivy::{doc, Index, Term};
/// # fn test() -> tantivy::Result<()> {
/// let mut schema_builder = Schema::builder();
/// let title = schema_builder.add_text_field("title", TEXT);
/// let schema = schema_builder.build();
/// let index = Index::create_in_ram(schema);
/// {
///     let mut index_writer = index.writer(3_000_000)?;
///     index_writer.add_document(doc!(
///         title => "The Name of the Wind",
///     ));
///     index_writer.add_document(doc!(
///         title => "The Diary of Muadib",
///     ));
///     index_writer.add_document(doc!(
///         title => "A Dairy Cow",
///     ));
///     index_writer.add_document(doc!(
///         title => "The Diary of a Young Girl",
///     ));
///     index_writer.commit()?;
/// }
/// let reader = index.reader()?;
/// let searcher = reader.searcher();
/// let query = TermQuery::new(
///     Term::from_field_text(title, "diary"),
///     IndexRecordOption::Basic,
/// );
/// let weighted_query = WeightedQuery::new(query, Score::new((1, 2.)));
/// let (top_docs, count) = searcher.search(&weighted_query, &(TopDocs::with_limit(2), Count))?;
/// assert_eq!(count, 2);
/// Ok(())
/// ```
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
