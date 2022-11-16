/// Copyright (c) 2021 Marcos Pontes
// This code is licensed under MIT license (see LICENSE for details)
use crate::eval::Evaluator;
use crate::memory_system::elements::ranklist::RankList;

///
/// Precision is the fraction of the documents retrieved that are relevant to the user's information need.
/// `precision = relevant_retrieved / retrieved`.
/// See [Wikipedia](https://en.wikipedia.org/wiki/Precision_and_recall#Precision) for more information.
///
#[derive(Debug, Clone)]
pub struct Precision {
    limit: usize,
}

impl Precision {
    ///
    /// Creates a new `Precision` instance.
    ///
    pub fn new(limit: usize) -> Precision {
        Precision { limit }
    }

    ///
    /// Get the limit K.
    ///
    pub fn limit(&self) -> usize {
        self.limit
    }

    ///
    /// Set the limit K.
    /// `limit` must be greater than 0.
    ///
    pub fn set_limit(&mut self, limit: usize) {
        self.limit = limit;
    }
}

impl Evaluator for Precision {
    ///
    /// Evaluates the precision of the given rank list.
    ///
    fn evaluate_ranklist(&self, ranklist: &RankList) -> f32 {
        let mut precision_score = 0.0f32;
        for i in 0..self.limit {
            match ranklist.get(i) {
                Ok(dp) => {
                    if dp.get_label() == 1 {
                        precision_score += 1.0;
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
        match self.limit {
            0 => 0.0,
            _ => precision_score / self.limit as f32,
        }
    }
}

impl ToString for Precision {
    fn to_string(&self) -> String {
        format!("P@{}", self.limit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::memory_system::elements::datapoint::DataPoint;
    use crate::memory_system::elements::feature::Feature;
    use crate::memory_system::elements::ranklist::RankList;
    use crate::rl;
    use crate::utils::randomize;

    use approx::relative_eq;

    #[test]
    fn test_precision() {
        let ranklist = rl!(
            (
                0,
                9,
                randomize::randomize_uniform(Feature::from(0f32), Feature::from(100f32), 20),
                "doc1"
            ),
            (
                1,
                9,
                randomize::randomize_uniform(Feature::from(0f32), Feature::from(100f32), 20),
                "doc2"
            ),
            (
                1,
                9,
                randomize::randomize_uniform(Feature::from(0f32), Feature::from(100f32), 20),
                "doc3"
            ),
            (
                0,
                9,
                randomize::randomize_uniform(Feature::from(0f32), Feature::from(100f32), 20),
                "doc4"
            ),
            (
                1,
                9,
                randomize::randomize_uniform(Feature::from(0f32), Feature::from(100f32), 20),
                "doc5"
            ),
            (
                0,
                9,
                randomize::randomize_uniform(Feature::from(0f32), Feature::from(100f32), 20),
                "doc6"
            )
        );

        let p1 = Precision::new(1);
        let mut p3 = Precision::new(3);
        let p5 = Precision::new(5);

        let p1_score = p1.evaluate_ranklist(&ranklist);
        let p3_score = p3.evaluate_ranklist(&ranklist);
        let p5_score = p5.evaluate_ranklist(&ranklist);

        assert!(relative_eq!(p1_score, 0.0, max_relative = 0.01f32));
        assert!(relative_eq!(p3_score, 0.66, max_relative = 0.01f32));
        assert!(relative_eq!(p5_score, 0.6, max_relative = 0.01f32));

        assert_eq!(p1.limit(), 1);
        assert_eq!(p3.limit(), 3);
        assert_eq!(p5.limit(), 5);

        p3.set_limit(2);
        assert_eq!(p3.limit(), 2);
        assert!(relative_eq!(
            p3.evaluate_ranklist(&ranklist),
            0.5,
            max_relative = 0.01f32
        ));
    }
}
