/// Copyright (c) 2021 Marcos Pontes
use crate::{datapoint::DataPoint, ranklist::RankList, DataSet};

/// The `Ranker` trait represents the basic behavior for
/// the models.
///
/// The models should be able to predict a `DataSet` and
/// rank based on the scores.
pub trait Ranker {
    /// Generates a score for a `DataPoint`.
    ///
    /// # Arguments
    /// * `datapoint` - The `DataPoint` to predict.
    ///
    /// # Returns
    /// The score for the `DataPoint`.
    fn predict(&self, datapoint: &DataPoint) -> f32;

    /// Perform ranking on a `RankList`.
    ///
    /// # Arguments
    /// * `ranklist` - The `RankList` to rank.
    ///
    /// # Returns
    /// The ranked `RankList`.
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

    /// Perform ranking on a `DataSet`.
    ///
    /// # Arguments
    /// * `dataset` - The `DataSet` to rank.
    fn rank_dataset(&self, dataset: &DataSet) {
        for ranklist in dataset.iter() {
            self.rank(ranklist);
        }
    }
}
