/// Copyright (c) 2021 Marcos Pontes
// This code is licensed under MIT license (see LICENSE for details)
use crate::{
    memory_system::elements::{datapoint::DataPoint, ranklist::RankList},
    DataSet,
};

/// Idea
/// Trait Ranker: predict + rank
/// Trait Learner: fit + other learning stuff
/// Trait Serializable: save to json
///
/// Models: T : Ranker + Learner + Serializable

///
/// The `Ranker` trait represents the basic behavior for
/// all models implemented in the lt.rs crate.
///
/// The models should be able to predict a `DataSet` and
/// rank based on the scores.
///
pub trait Ranker {
    ///
    /// Generates a score for a `DataPoint`.
    ///
    fn predict(&self, datapoint: &DataPoint) -> f32;

    ///
    /// Perform ranking on a `RankList`.
    ///
    fn rank(&self, ranklist: &RankList) {
        let mut score_per_index: Vec<(usize, f32)> = ranklist
            .into_iter()
            .enumerate()
            .map(|(i, dp)| (i, self.predict(&dp)))
            .collect();

        // Sort by score
        score_per_index.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Reorder the ranklist based on the index of the sorted score
        ranklist
            .permute(score_per_index.iter().map(|&(i, _)| i).collect())
            .unwrap();
    }

    ///
    /// Perform ranking on a `DataSet`.
    ///
    fn rank_dataset(&self, dataset: &DataSet) {
        for ranklist in dataset.iter() {
            self.rank(ranklist);
        }
    }
}
