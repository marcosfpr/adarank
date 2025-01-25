/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///
use crate::ranker::Ranker;

///
/// A WeakRanker can be interpreted as a stump in an ensemble of rankers.
/// Usually, the weak ranker is dummy, and it only evaluates the `RankList`
/// considering a single feature inside the `DataPoint`s.
///
#[derive(Debug, Clone)]
pub struct WeakRanker {
    ///
    /// The feature index of the feature to be used in the ranking.
    ///
    pub feature_id: usize,
}

impl WeakRanker {
    ///
    /// Creates a new WeakRanker.
    ///
    pub fn new(feature_id: usize) -> Self {
        WeakRanker { feature_id }
    }
}

impl Ranker for WeakRanker {
    fn predict(&self, datapoint: &crate::datapoint::DataPoint) -> f32 {
        match datapoint.get_feature(self.feature_id) {
            Ok(value) => *value,
            Err(_) => 0.0f32,
        }
    }
}
