/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///
pub mod precision;
pub mod map;

use crate::ranklist::RankList;
use crate::DataSet;
use crate::error::LtrError;

///
/// The library supported metrics must implement this trait.
///
trait Evaluator {
    ///
    /// Evaluates a `DataSet`
    ///
    /// # Arguments
    ///
    /// * `dataset` - The `DataSet` to be evaluated.
    ///
    /// # Returns
    ///
    /// The metric value.
    ///
    /// todo: refactor  all this shit
    fn evaluate_dataset(&self, dataset: &DataSet) -> Result<f64, LtrError> {
        let mut score = 0.0f64;
        for ranklist in dataset {
            match self.evaluate_ranklist(ranklist) {
                Ok(value) => {
                    score += value;
                }
                Err(_) => { continue; } // todo: think if this have to be exception safe.
            }
        }
        return Ok(score / dataset.len() as f64);
    }

    ///
    /// Evaluates a `RankList`. Notice that the `RankList` must be ordered by relevance!
    ///
    /// # Arguments
    ///
    /// * `ranklist` - The `RankList` to be evaluated.
    ///
    /// # Returns
    ///
    /// The metric value.
    ///
    fn evaluate_ranklist(&self, ranklist: &RankList) -> Result<f64, LtrError>;
}
