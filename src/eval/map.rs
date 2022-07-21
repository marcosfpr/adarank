/// Copyright (c) 2021 Marcos Pontes
// This code is licensed under MIT license (see LICENSE for details)
use crate::eval::Evaluator;
use crate::ranklist::RankList;

///
/// MAP (Mean Average Precision) for a set of queries is the mean of the average precision
/// scores for each query.
/// The average precision score is the sum of the precision scores for each k, divided by
/// the number of positive labels.
/// See [Medium](https://towardsdatascience.com/breaking-down-mean-average-precision-map-ae462f623a52) for more information.
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
    use crate::datapoint::DataPoint;
    use crate::ranklist::RankList;
    use crate::rl;
    use crate::utils::randomize;

    use approx::relative_eq;

    #[test]
    fn test_map() {
        let ranklist = rl!(
            (0, 9, randomize::randomize_uniform(0f32, 100f32, 20), "doc1"),
            (1, 9, randomize::randomize_uniform(0f32, 100f32, 20), "doc2"),
            (1, 9, randomize::randomize_uniform(0f32, 100f32, 20), "doc3"),
            (0, 9, randomize::randomize_uniform(0f32, 100f32, 20), "doc4"),
            (1, 9, randomize::randomize_uniform(0f32, 100f32, 20), "doc5"),
            (0, 9, randomize::randomize_uniform(0f32, 100f32, 20), "doc6")
        );

        let map = MAP;

        let map_score = map.evaluate_ranklist(&ranklist);

        assert!(relative_eq!(map_score, 0.588, max_relative = 0.01f64));
    }
}
