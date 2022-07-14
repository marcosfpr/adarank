/// Copyright (c) 2021 Marcos Pontes
// This code is licensed under MIT license (see LICENSE for details)

use crate::eval::Evaluator;
use crate::ranklist::RankList;

///
/// MAP (Mean Average Precision) for a set of queries is the mean of the average precision
/// scores for each query.
/// See [Wikipedia](https://en.wikipedia.org/wiki/Evaluation_measures_(information_retrieval)#Mean_average_precision) for more information.
///
#[derive(Debug, Clone)]
struct MAP;

impl Evaluator for MAP {
    ///
    /// Evaluates the MAP for a set of queries.
    ///
    fn evaluate_ranklist(&self, ranklist: &RankList) -> f64 {
        let mut average_precision = 0.0f64;
        let mut num_relevant_docs = 0;
        for i in 0..ranklist.len() {
            match ranklist.get(i) {
                Ok(dp) => {
                    if dp.get_label() > 0 {
                        num_relevant_docs += 1;
                        average_precision += num_relevant_docs as f64 / (i as f64 + 1.0);
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
        match num_relevant_docs {
            0 => 0.0,
            _ => average_precision / num_relevant_docs as f64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ranklist::RankList;
    use crate::datapoint::DataPoint;

    #[test]
    fn test_map() {

    }
}