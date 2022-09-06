/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///
use crate::ranker::Ranker;

///
/// Defines a stumb of an ensemble tree as a weak Ranker.
/// It just retrieves the score of a DataPoint as the value of an isolated feature.
/// 
#[derive(Debug, Clone)]
pub struct WeakRanker {
    /// Feature id to be used as the final score for each DataPoint.
    pub feature_id: usize,
}

impl WeakRanker {
    ///
    /// Creates a new WeakRanker given a feature id.
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
