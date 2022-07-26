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

struct AdaRank {
    training_dataset: DataSet,
    validation_dataset: Option<DataSet>,
    scorer: Box<dyn Evaluator>,
    pub iter: u64,
    max_consecutive_selections: usize,
    consecutive_selections: usize,
    previous_feature: isize,
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
        features: Vec<usize>,
        validation_dataset: Option<DataSet>,
    ) -> Self {
        let rankers = Vec::new();
        let best_rankers = Vec::new();
        let ranker_weights = Vec::new();
        let best_weights = Vec::new();
        let used_features = HashSet::new();

        let sample_weights = AdaRank::initialize_weights(training_dataset.len());
        let tcfg = AdaRank::table_config();

        AdaRank {
            training_dataset,
            validation_dataset,
            scorer,
            iter,
            max_consecutive_selections,
            consecutive_selections: 0,
            previous_feature: -1,
            tolerance,
            score_training: 0.0,
            score_validation: 0.0,
            features,
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
                format!("{}", training_score).as_str(),
                format!("{}", improvement).as_str(),
                format!("{}", validation_score).as_str(),
                format!("{}", validation_improvement).as_str(),
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
            "{}",
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
            logging::log_table_row(
                vec![
                    format!("{}", self.score_training).as_str(),
                    format!("{}", self.score_validation).as_str(),
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

    fn evaluate_weak_ranker(&mut self, ranker: &WeakRanker) -> f32 {
        let mut score = 0.0;
        for (i, sample) in self.training_dataset.iter_mut().enumerate() {
            ranker.rank(sample);
            score += self.scorer.evaluate_ranklist(sample) * self.sample_weights[i];
        }
        score
    }

    fn select_weak_ranker(&mut self) -> WeakRanker {
        let mut best_score = -1.0;
        let mut best_feature = 0;

        for feature in self.features.iter() {
            if self.used_features.contains(feature) {
                continue;
            }
            let mut ranker = WeakRanker::new(*feature);
            let score = self.evaluate_weak_ranker(&ranker);
            if score > best_score {
                best_score = score;
                best_feature = *feature;
            }
        }
        WeakRanker::new(best_feature)
    }

    fn learn() {
        todo!()
    }
}

impl Learner for AdaRank {
    fn fit(&mut self) -> Result<(), crate::error::LtrError> {
        log::debug!("{}", self.debug_header());

        log::debug!("{}", self.get_results());
        Ok(())
    }

    fn score(&self) -> Result<f32, crate::error::LtrError> {
        todo!()
    }

    fn validation_score(&self) -> Result<f32, crate::error::LtrError> {
        todo!()
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
