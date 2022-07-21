/// Copyright (c) 2021 Marcos Pontes
// This code is licensed under MIT license (see LICENSE for details)

use crate::{datapoint::DataPoint, ranklist::RankList, DataSet};

///
/// The Ranker trait represents the basic behavior for
/// all models implemented in the lt.rs crate.
/// 
/// The capabilities of a LTR ranker is to generate a score for
///  a single `DataPoint` and to provide an entire rank for
///  a `RankList` or a `DataSet`.
pub trait Ranker {

    ///
    /// Generates a score for a `DataPoint`.
    /// 
    fn predict(&self, datapoint: &DataPoint) -> f64;

    ///
    /// Perform ranking on a `RankList`.
    /// 
    fn rank(&self, ranklist: &mut RankList) {
        let mut score_per_index: Vec<(usize, f64)> = ranklist
            .into_iter()
            .enumerate()
            .map(|(i, dp)| (i, self.predict(&dp)))
            .collect();
        
        // Sort by score
        score_per_index.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Reorder the ranklist based on the index of the sorted score
        ranklist.permute(score_per_index.iter().map(|&(i, _)| i).collect());
    }


    ///
    /// Perform ranking on a `DataSet`.
    /// 
    fn rank_dataset(&self, dataset: &mut DataSet) {
        for ranklist in dataset.iter_mut() {
            self.rank(ranklist);
        }
    }

}