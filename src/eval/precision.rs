/// Copyright (c) 2021 Marcos Pontes
// This code is licensed under MIT license (see LICENSE for details)

use crate::error::LtrError;
use crate::eval::Evaluator;
use crate::ranklist::RankList;

///
/// Precision is the fraction of the documents retrieved that are relevant to the user's information need.
/// `precision = relevant_retrieved / retrieved`.
/// See [Wikipedia](https://en.wikipedia.org/wiki/Precision_and_recall#Precision) for more information.
///
#[derive(Debug, Clone)]
struct Precision {
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
    fn evaluate_ranklist(&self, ranklist: &RankList) -> f64 {
        let mut precision_score = 0.0f64;
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
            _ => precision_score / self.limit as f64,
        }
    }
}
