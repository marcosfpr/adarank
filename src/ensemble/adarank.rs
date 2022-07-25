/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///
use std::collections::HashSet;

use super::weak::WeakRanker;
use crate::{
    learner::{
        DatasetConfigurable, FeaturesConfigurable, FileSerializable, Learner, MetricConfigurable,
    },
    DataSet, eval::Evaluator,
};

struct AdaRank {
    training_dataset: DataSet,
    validation_dataset: Option<DataSet>,
    scorer: Box<dyn Evaluator>,
    iter: u64,
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
        }
    }

    fn initialize_weights(len: usize) -> Vec<f32> {
        // Create a vector of size `len` with values 1.0/`len`
        let mut weights = Vec::new();   
        for _ in 0..len {
            weights.push(1.0 / len as f32);
        }
        weights
    }
        
}


impl Learner for AdaRank {
    fn fit(&mut self) -> Result<(), crate::error::LtrError> {
        todo!()
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
