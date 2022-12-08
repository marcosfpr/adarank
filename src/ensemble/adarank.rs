/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///
use std::collections::HashSet;

use colored::Color;

use super::weak::WeakRanker;

use crate::{
    eval::Evaluator,
    learner::{
        DatasetConfigurable, FeaturesConfigurable, FileSerializable, Learner, MetricConfigurable,
    },
    memory_system::elements::dataset::DataSet,
    ranker::Ranker,
    utils::logging::{Alignment, TableConfig, TableLogger},
};

///
/// The basic idea of AdaRank is constructing “weak rankers” repeatedly based on reweighted
/// training queries and linearly combining the weak rankers for making ranking predictions.
/// In learning, AdaRank minimizes a loss function directly defined on performance measures.
/// The details of AdaRank can be found in the paper “AdaRank: A Boosting Algorithm for Information Retrieval
///
pub struct AdaRank {
    ///
    /// Training dataset.
    ///
    training_dataset: DataSet,
    ///
    /// Optional dataset to be used for validation.
    ///
    validation_dataset: Option<DataSet>,
    ///
    /// Pointer to a evaluator.
    ///
    scorer: Box<dyn Evaluator>,
    ///
    /// The number of iterations to be performed.
    ///
    pub iter: u64,
    ///
    /// Maximum number of consecutive feature selection
    ///
    max_consecutive_selections: usize,
    ///
    /// Current number of consecutive feature selection
    ///
    consecutive_selections: usize,
    ///
    /// Previous selected feature.
    ///
    previous_feature: usize,
    ///
    /// Tolerance criteria to stop the algorithm.
    ///
    pub tolerance: f32,
    /// The model scoring during the training phase.
    ///
    /// Training score of the model.
    ///
    pub score_training: f32,
    ///
    /// Validation score of the model.
    ///
    pub score_validation: f32,
    ///
    /// Subset of features to be used in the model.
    ///
    features: Vec<usize>,
    ///
    /// Previous training score.
    ///
    previous_traning_score: f32,
    ///
    /// Previous validation score.
    ///
    previous_validation_score: f32,
    ///
    /// Sample's weights. It indicates the importance of each sample
    /// in each iteration of the training process.
    ///
    sample_weights: Vec<f32>,
    ///
    /// The amount of say for each stump of the ensemble.
    ///
    ranker_weights: Vec<f32>,
    ///
    /// Best model's weights. It indicates the importance of each stump during the training process.
    ///
    best_weights: Vec<f32>,
    ///
    /// Best `WeakRanker`s of the ensemble.
    ///
    rankers: Vec<WeakRanker>,
    ///
    /// Best `WeakRanker`s found during the training process.
    ///
    best_rankers: Vec<WeakRanker>,
    ///
    /// Features already saturated.
    ///
    used_features: HashSet<usize>,
    ///
    /// Logger.
    ///
    logger: TableLogger,
}

impl AdaRank {
    ///
    /// Create a new `AdaRank` instance.
    ///
    pub fn new(
        training_dataset: DataSet,
        scorer: Box<dyn Evaluator>,
        iter: u64,
        max_consecutive_selections: usize,
        tolerance: f32,
        features: Option<Vec<usize>>,
        validation_dataset: Option<DataSet>,
    ) -> Self {
        let rankers = Vec::new();
        let best_rankers = Vec::new();
        let ranker_weights = Vec::new();
        let best_weights = Vec::new();
        let used_features = HashSet::new();

        let sample_weights = AdaRank::initialize_weights(training_dataset.len());
        let tcfg = AdaRank::table_config();

        // If None, use all features -> range(0, training_dataset[0].len())
        let features_used = match features {
            Some(ft) => ft,
            None => (1..training_dataset[0].len() + 1).collect(),
        };

        AdaRank {
            training_dataset,
            validation_dataset,
            scorer,
            iter,
            max_consecutive_selections,
            consecutive_selections: 0,
            previous_feature: usize::MAX,
            tolerance,
            score_training: 0.0,
            score_validation: 0.0,
            features: features_used,
            previous_traning_score: 0.0,
            previous_validation_score: 0.0,
            sample_weights,
            ranker_weights,
            best_weights,
            rankers,
            best_rankers,
            used_features,
            logger: TableLogger::new(tcfg),
        }
    }

    fn table_config() -> TableConfig {
        TableConfig::new(vec![7, 8, 9, 9, 9, 9, 9], (2, 2), Alignment::Center)
    }

    fn debug_header(&self) -> String {
        self.logger.log(
            vec![
                "#Iter",
                "Feature",
                format!("{}-T", self.scorer.to_string()).as_str(),
                "Improve-T",
                format!("{}-V", self.scorer.to_string()).as_str(),
                "Improve-V",
                "Status",
            ],
            Some(Color::Cyan),
        )
    }

    fn debug_line(
        &self,
        current_it: usize,
        feature: usize,
        training_score: f32,
        improvement: f32,
        validation_score: f32,
        validation_improvement: f32,
        status: &str,
    ) -> String {
        self.logger.log(
            vec![
                format!("{}", current_it).as_str(),
                format!("{}", feature).as_str(),
                format!("{:.5}", training_score).as_str(),
                format!("{:.5}", improvement).as_str(),
                format!("{:.5}", validation_score).as_str(),
                format!("{:.5}", validation_improvement).as_str(),
                status,
            ],
            None,
        )
    }

    ///
    /// Get the training results summary.
    ///
    pub fn log_results(&self) {
        let results_config = TableConfig::new(vec![9, 9], (2, 2), Alignment::Center);
        let table_logger = TableLogger::new(results_config);

        log::info!(
            "{}",
            table_logger.log(
                vec![
                    format!("{}-T", self.scorer.to_string()).as_str(),
                    format!("{}-V", self.scorer.to_string()).as_str(),
                ],
                Some(Color::Cyan),
            )
        );
        log::info!(
            "{}",
            table_logger.log(
                vec![
                    format!("{:.5}", self.score_training).as_str(),
                    format!("{:.5}", self.score_validation).as_str(),
                ],
                None,
            )
        );
    }

    fn initialize_weights(len: usize) -> Vec<f32> {
        // Create a vector of size `len` with values 1.0/`len`
        let mut weights = Vec::new();
        for _ in 0..len {
            weights.push(1.0 / len as f32);
        }
        weights
    }

    fn evaluate_weak_ranker(&self, ranker: &WeakRanker) -> f32 {
        let mut score = 0.0;
        for (i, sample) in self.training_dataset.iter().enumerate() {
            score += self.scorer.evaluate_ranklist(&ranker.rank(sample)) * self.sample_weights[i];
        }
        score
    }

    fn select_weak_ranker(&mut self) -> Option<WeakRanker> {
        let mut best_score = -1.0;
        let mut best_feature = 0;

        for feature in self.features.iter() {
            if self.used_features.contains(feature) {
                continue;
            }
            let ranker = WeakRanker::new(*feature);
            let score = self.evaluate_weak_ranker(&ranker);
            if score > best_score {
                best_score = score;
                best_feature = *feature;
            }
        }

        if best_score < 0.0 {
            return None;
        }

        Some(WeakRanker::new(best_feature))
    }

    fn learn(&mut self) {
        for it in 0..self.iter {
            // 1st step: select a weak ranker
            let best_weak_ranker = match self.select_weak_ranker() {
                Some(ranker) => ranker,
                None => {
                    log::error!("No weak ranker selected");
                    break;
                }
            };

            // 2nd step: evaluate the weak ranker (amount to say)
            let mut num = 0.0f32;
            let mut denom = 0.0f32;
            for (ranklist, weight) in self
                .training_dataset
                .iter_mut()
                .zip(self.sample_weights.iter())
            {
                let score = self
                    .scorer
                    .evaluate_ranklist(&best_weak_ranker.rank(ranklist));
                num += (1.0 + score) * *weight;
                denom += (1.0 - score) * *weight;
            }

            // TODO: that's a work around for negative feature values
            let amount_to_say = 0.5 * (num.abs() / denom.abs()).log10();

            // 3rd step: update the weights
            self.rankers.push(best_weak_ranker.clone());
            self.ranker_weights.push(amount_to_say);

            // 4th step: evaluate the ensemble on the training and validation dataset

            let mut training_score = 0.0f32;
            let mut total_score = 0.0f32;

            let mut train_scores_list = Vec::new();
            train_scores_list.reserve(self.training_dataset.len());

            for ranklist in self.training_dataset.iter() {
                let score = self.scorer.evaluate_ranklist(&self.rank(ranklist));
                let exp_score = (-score).exp();

                training_score += score;
                total_score += exp_score;

                train_scores_list.push(exp_score);
            }

            training_score /= self.training_dataset.len() as f32;
            let delta = training_score + self.tolerance - self.previous_traning_score;

            let mut status = if delta > 0.0 { "OK" } else { "BAD" };

            let selected_feature = best_weak_ranker.feature_id;

            if self.previous_feature == selected_feature {
                self.consecutive_selections += 1;
                if self.consecutive_selections == self.max_consecutive_selections {
                    status = "SATURED";
                    self.consecutive_selections = 0;
                    self.used_features.insert(selected_feature);
                }
            }

            self.previous_feature = selected_feature;

            let mut val_score = 0.0f32;
            if let Some(val_dataset) = &self.validation_dataset {
                if !val_dataset.is_empty() && it % 1 == 0 {
                    val_score = match self
                        .scorer
                        .evaluate_dataset(&self.rank_dataset(val_dataset))
                    {
                        Ok(score) => score,
                        Err(e) => {
                            log::error!("Error evaluating validation dataset: {}", e);
                            0.0
                        }
                    };
                    if val_score > self.score_validation {
                        self.score_validation = val_score;
                        self.best_rankers = self.rankers.clone();
                        self.best_weights = self.ranker_weights.clone();
                    }
                }
            }

            let train_improvement = training_score - self.previous_traning_score;
            let validation_improvement = val_score - self.previous_validation_score;

            log::debug!(
                "{}",
                self.debug_line(
                    it as usize,
                    selected_feature,
                    training_score,
                    train_improvement,
                    val_score,
                    validation_improvement,
                    status,
                )
            );

            if delta <= 0.0 {
                self.rankers.pop();
                self.ranker_weights.pop();
                break;
            }

            self.previous_traning_score = training_score;
            self.previous_validation_score = val_score;

            // 5th step: update the weights distribution
            for (weight, score) in self.sample_weights.iter_mut().zip(train_scores_list.iter()) {
                *weight *= (-amount_to_say * score).exp() / total_score;
            }
        }
    }
}

impl Learner for AdaRank {
    fn fit(&mut self) -> Result<(), crate::error::LtrError> {
        log::debug!("{}", self.debug_header());

        self.learn();

        if !self.best_rankers.is_empty() {
            self.rankers = std::mem::take(&mut self.best_rankers);
            self.ranker_weights = std::mem::take(&mut self.best_weights);
        }

        if self.rankers.is_empty() {
            return Err(crate::error::LtrError::NoRankers);
        }

        self.score_training = self
            .scorer
            .evaluate_dataset(&self.rank_dataset(&self.training_dataset))?;

        match &self.validation_dataset {
            Some(dataset) => {
                self.score_validation = self
                    .scorer
                    .evaluate_dataset(&self.rank_dataset(dataset))
                    .unwrap_or_else(|e| {
                        log::error!("Error evaluating training dataset: {}", e);
                        0.0
                    });
            }
            None => {
                self.score_validation = 0.0;
            }
        }

        self.log_results();
        Ok(())
    }

    fn score(&self) -> Result<f32, crate::error::LtrError> {
        if self.rankers.is_empty() {
            return Err(crate::error::LtrError::NoRankers);
        }
        Ok(self.score_training)
    }

    fn validation_score(&self) -> Result<f32, crate::error::LtrError> {
        if self.rankers.is_empty() {
            return Err(crate::error::LtrError::NoRankers);
        }
        Ok(self.score_validation)
    }
}

impl Ranker for AdaRank {
    fn predict(&self, datapoint: &crate::memory_system::elements::datapoint::DataPoint) -> f32 {
        let mut score = 0.0;
        for (ranker, weight) in self.rankers.iter().zip(self.ranker_weights.iter()) {
            let feature_value: f32 = match datapoint.get_feature(ranker.feature_id) {
                Ok(value) => **value,
                Err(e) => {
                    log::error!("Error getting feature value: {}", e);
                    0.0
                }
            };
            score += feature_value * weight;
        }
        score
    }
}

impl FileSerializable for AdaRank {
    fn save_to_file(&self, _path: &str) -> Result<(), crate::error::LtrError> {
        todo!()
    }

    fn load_from_file(&mut self, _path: &str) -> Result<(), crate::error::LtrError> {
        todo!()
    }
}

impl FeaturesConfigurable for AdaRank {
    fn set_features(&mut self, _features: Vec<usize>) {
        todo!()
    }

    fn get_features(&self) -> Vec<usize> {
        todo!()
    }
}

impl MetricConfigurable for AdaRank {
    fn set_metric(&mut self, _metric: &dyn crate::eval::Evaluator) {
        todo!()
    }
}

impl DatasetConfigurable for AdaRank {
    fn set_train_dataset(&mut self, _dataset: DataSet) {
        todo!()
    }

    fn set_validation_dataset(&mut self, _dataset: DataSet) {
        todo!()
    }
}
