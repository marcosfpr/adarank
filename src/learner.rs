/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///

use crate::{error::LtrError, DataSet, eval::Evaluator, ranker::Ranker};

///
/// This trait represents the basic behavior for
/// all models implemented in the lt.rs crate.
/// 
/// The models should be able to fit a `DataSet` and
/// rank a `DataPoint`.
/// 
pub trait Learner : Ranker + DatasetConfigurable + MetricConfigurable + FeaturesConfigurable + FileSerializable
{

    ///
    /// Fit a `DataSet` to the model.
    /// 
    fn fit(&mut self) -> Result<(), LtrError>;


    ///
    /// The `Learner`s should retrieve the training process score.
    /// 
    fn score(&self) -> Result<f32, LtrError>;

    ///
    /// The `Learner`s should retrieve the validation process score.
    /// 
    fn validation_score(&self) -> Result<f32, LtrError>;

}


///
/// The `Learner`s should have a training and a optional validation `DataSet`.
/// 
/// The training `DataSet` is used to train the model.
/// The validation `DataSet` is used to validate the model during training.
/// TODO: document this
///  
pub trait  DatasetConfigurable{
    
    ///
    /// The `Learner`s should have a training and a optional validation `DataSet`.
    /// 
    /// The training `DataSet` is used to train the model.
    /// The validation `DataSet` is used to validate the model during training.
    ///
    fn set_train_dataset(&mut self, dataset: DataSet);

    ///
    /// The `Learner`s should have a training and a optional validation `DataSet`.
    /// 
    /// The training `DataSet` is used to train the model.
    /// The validation `DataSet` is used to validate the model during training.
    /// 
    fn set_validation_dataset(&mut self, dataset: DataSet);

}

///
/// The `Learner`s should allow the user to customize
/// the metrics used to evaluate the model.
/// 
/// TODO: document this
pub trait MetricConfigurable{
    
    ///
    /// The `Learner`s should allow the user to customize
    /// the metrics used to evaluate the model.
    /// 
    fn set_metric(&mut self, metric: &dyn Evaluator);

}

    
///
/// The `Learner`s should allow the user to customize
/// the features used to train the model.
/// TODO: document this
pub trait FeaturesConfigurable{
    
    ///
    /// The `Learner`s should allow the user to customize
    /// the features used to train the model.
    /// 
    fn set_features(&mut self, features: Vec<usize>);

    ///
    /// The `Learner`s should retrieve the features used to train the model.
    /// 
    fn get_features(&self) -> Vec<usize>;

}

///
/// The `Learner`s should allow the user to save the model to a file.
/// TODO: document this
pub trait FileSerializable {

    ///
    /// The `Learner`s should allow the user to save the model to a file.
    /// 
    fn save_to_file(&self, path: &str) -> Result<(), LtrError>;

    ///
    /// The `Learner`s should allow the user to load the model from a file.
    /// 
    fn load_from_file(&mut self, path: &str) -> Result<(), LtrError>;
}