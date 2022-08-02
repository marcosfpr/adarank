/// Copyright (c) 2021 Marcos Pontes
// This code is licensed under MIT license (see LICENSE for details)

///
/// Metric MAP (Mean Average Precision).
/// 
pub mod map;

///
/// Metric P@k (Precision at k).
/// 
pub mod precision;

use crate::error::LtrError;
use crate::ranklist::RankList;
use crate::DataSet;

///
///
/// The Evaluator trait allows us to create or own ways to assess
/// the ranking effectiveness. In the literature, many different evaluators
/// were already proposed: NDCG, MAP, F1, Precision, Recall, etc.
///
/// Currently we  already have some of these evaluators implemented.
///
/// Under the hood, to implement an  `Evaluator` you need to
/// implement the function `evaluate_ranklist`, which evaluates the results
/// generated  from a single `RankList`.
///
pub trait Evaluator: ToString {
    ///
    /// Evaluates a `DataSet`
    ///
    /// # Arguments
    ///
    /// * `dataset` - The `DataSet` to be evaluated.
    ///
    /// # Returns
    /// Average of the metric defined on the `evaluate_ranklist` function.
    ///
    fn evaluate_dataset(&self, dataset: &DataSet) -> Result<f32, LtrError> {
        if dataset.is_empty() {
            return Err(LtrError::EvaluationError(
                "Error in Evaluator::evaluate_dataset: the dataset is empty.",
            ));
        }
        let mut score = 0.0f32;
        for ranklist in dataset {
            score += self.evaluate_ranklist(ranklist);
        }
        Ok(score / dataset.len() as f32)
    }

    ///
    /// Evaluates a `RankList` previously ordered by relevance.
    ///
    /// Notice that the evaluation is error safe, meaning that if an error occurs during the
    /// evaluation, the function will return `0.0`.
    ///
    /// # Arguments
    ///
    /// * `ranklist` - The `RankList` to be evaluated.
    ///
    /// # Returns
    ///
    /// The metric value.
    ///
    fn evaluate_ranklist(&self, ranklist: &RankList) -> f32;
}

// todo: test this module
