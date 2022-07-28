/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///
use std::collections::HashSet;

use super::weak::WeakRanker;
use crate::{
    eval::Evaluator,
    learner::{
        DatasetConfigurable, FeaturesConfigurable, FileSerializable, Learner, MetricConfigurable,
    },
    ranker::Ranker,
    utils::{
        format::{Alignment, TableConfig},
        logging,
    },
    DataSet,
};

pub struct AdaRank {
    training_dataset: DataSet,
    validation_dataset: Option<DataSet>,
    scorer: Box<dyn Evaluator>,
    pub iter: u64,
    max_consecutive_selections: usize,
    consecutive_selections: usize,
    previous_feature: usize,
    pub tolerance: f32,
    pub score_training: f32,
    pub score_validation: f32,
    features: Vec<usize>,
    previous_traning_score: f32,
    previous_validation_score: f32,
    sample_weights: Vec<f32>,
    ranker_weights: Vec<f32>,
    best_weights: Vec<f32>,
    rankers: Vec<WeakRanker>,
    best_rankers: Vec<WeakRanker>,
    used_features: HashSet<usize>,
    table_config: TableConfig,
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
            None => (1..training_dataset[0].len()+1).collect(),
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
            table_config: tcfg,
        }
    }

    fn table_config() -> TableConfig {
        TableConfig::new(
            vec![7, 8, 9, 9, 9, 9, 9],
            (2, 2),
            Alignment::Center,
            true,
            true,
            true,
        )
    }

    fn debug_header(&self) -> String {
        logging::log_table_header(
            vec![
                "#Iter",
                "Feature",
                format!("{}-T", self.scorer.to_string()).as_str(),
                "Improve-T",
                format!("{}-V", self.scorer.to_string()).as_str(),
                "Improve-V",
                "Status",
            ],
            &self.table_config,
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
        logging::log_table_row(
            vec![
                format!("{}", current_it).as_str(),
                format!("{}", feature).as_str(),
                format!("{:.5}", training_score).as_str(),
                format!("{:.5}", improvement).as_str(),
                format!("{:.5}", validation_score).as_str(),
                format!("{:.5}", validation_improvement).as_str(),
                status,
            ],
            &self.table_config,
        )
    }

    pub fn get_results(&self) -> String {
        let results_table =
            TableConfig::new(vec![9, 9], (2, 2), Alignment::Center, true, true, true);

        let mut results = String::new();
        results.push_str(&format!(
            "{}\n",
            logging::log_table_header(
                vec![
                    format!("{}-T", self.scorer.to_string()).as_str(),
                    format!("{}-V", self.scorer.to_string()).as_str(),
                ],
                &results_table,
            )
        ));
        results.push_str(&format!(
            "{}",
            logging::log_shifted_table_row(
                vec![
                    format!("{:.5}", self.score_training).as_str(),
                    format!("{:.5}", self.score_validation).as_str(),
                ],
                &results_table,
            )
        ));
        results
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
            ranker.rank(sample);
            score += self.scorer.evaluate_ranklist(sample) * self.sample_weights[i];
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
        
        for it in 0..self.iter{

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
            for (ranklist, weight) in self.training_dataset.iter_mut().zip(self.sample_weights.iter()) {
                best_weak_ranker.rank(ranklist);
                let score = self.scorer.evaluate_ranklist(ranklist);
                num += (1.0 + score) * *weight;
                denom += (1.0 - score)* *weight;
            }
            
            let amount_to_say = 0.5 * (num / denom).log10();
            
            // 3rd step: update the weights
            self.rankers.push(best_weak_ranker.clone());
            self.ranker_weights.push(amount_to_say);

            // 4th step: evaluate the ensemble on the training and validation dataset

            let mut training_score = 0.0f32;
            let mut total_score = 0.0f32;

            let mut train_scores_list = Vec::new();
            train_scores_list.reserve(self.training_dataset.len());

            for ranklist in self.training_dataset.iter() {
                self.rank(ranklist);

                let score = self.scorer.evaluate_ranklist(ranklist);
                let exp_score = (-score).exp();

                training_score += score;
                total_score += exp_score;

                train_scores_list.push(exp_score);

            }

            training_score /= self.training_dataset.len() as f32;
            let delta = training_score + self.tolerance - self.previous_traning_score;

            let mut status = if delta > 0.0 {
                "OK"
            } else {
                "BAD"
            };

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
                    self.rank_dataset(val_dataset);
                    val_score = match self.scorer.evaluate_dataset(val_dataset) {
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

            log::debug!("{}", self.debug_line(
                it as usize,
                selected_feature,
                training_score,
                train_improvement,
                val_score,
                validation_improvement,
                status,
            ));

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

        self.rank_dataset(&self.training_dataset);
        self.score_training = self.scorer.evaluate_dataset(&self.training_dataset)?;

        match &self.validation_dataset {
            Some(dataset) => {
                self.rank_dataset(dataset);
                self.score_validation = self.scorer.evaluate_dataset(&self.training_dataset).unwrap_or_else(|e| {
                    log::error!("Error evaluating training dataset: {}", e);
                    0.0
                });
            }
            None => {
                self.score_validation = 0.0;
            }
        }

        log::debug!("{}", self.get_results());
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
    fn predict(&self, datapoint: &crate::datapoint::DataPoint) -> f32 {
        let mut score = 0.0;
        for (ranker, weight) in self.rankers.iter().zip(self.ranker_weights.iter()) {
            let feature_value: f32 = match datapoint.get_feature(ranker.feature_id) {
                Ok(value) => *value,
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
    fn save_to_file(&self, path: &str) -> Result<(), crate::error::LtrError> {
        todo!()
    }

    fn load_from_file(&mut self, path: &str) -> Result<(), crate::error::LtrError> {
        todo!()
    }
}

impl FeaturesConfigurable for AdaRank {
    fn set_features(&mut self, features: Vec<usize>) {
        todo!()
    }

    fn get_features(&self) -> Vec<usize> {
        todo!()
    }
}

impl MetricConfigurable for AdaRank {
    fn set_metric(&mut self, metric: &dyn crate::eval::Evaluator) {
        todo!()
    }
}

impl DatasetConfigurable for AdaRank {
    fn set_train_dataset(&mut self, dataset: DataSet) {
        todo!()
    }

    fn set_validation_dataset(&mut self, dataset: DataSet) {
        todo!()
    }
}
