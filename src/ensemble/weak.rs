use crate::ranker::Ranker;

/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///

#[derive(Debug, Clone)]
pub struct WeakRanker {
    pub feature_id: usize,
}

impl WeakRanker {
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
