/// Copyright (c) 2021 Marcos Pontes
// This code is licensed under MIT license (see LICENSE for details)
use crate::memory_system::elements::{
    datapoint::DataPoint,
    dataset::{DataSet, DataSetPermutation},
    ranklist::{RankList, RankListPermutation},
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
    fn rank<'a>(&self, ranklist: &'a RankList) -> RankListPermutation<'a> {
        let mut score_per_index: Vec<(usize, f32)> = ranklist
            .into_iter()
            .enumerate()
            .map(|(i, dp)| (i, self.predict(&dp)))
            .collect();

        // Sort by score
        score_per_index.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

        RankListPermutation {
            permutation: score_per_index
                .iter()
                .map(|(i, _)| *i)
                .collect::<Vec<usize>>(),
            ranklist: ranklist,
        }
    }

    ///
    /// Perform ranking on a `DataSet`. Quite expensive, use carefully.
    ///
    fn rank_dataset<'a>(&self, dataset: &'a DataSet) -> DataSetPermutation<'a> {
        let mut permutations = Vec::new();
        for ranklist in dataset.iter() {
            permutations.push(self.rank(ranklist));
            // ranklist
            //     .permute(rlp.permutation.iter().map(|&(i, _)| i).collect())
            //     .unwrap();
        }
        permutations
    }
}
